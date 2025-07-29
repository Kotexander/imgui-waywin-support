use imgui_vulkano_renderer::Renderer;
use imgui_waywin_support::WaywinPlatform;
use std::sync::Arc;
use vulkano::{
    Validated, VulkanError, VulkanLibrary,
    command_buffer::{
        AutoCommandBufferBuilder, CommandBufferUsage, PrimaryCommandBufferAbstract,
        RenderPassBeginInfo, SubpassBeginInfo, allocator::StandardCommandBufferAllocator,
    },
    descriptor_set::allocator::StandardDescriptorSetAllocator,
    device::{Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo, QueueFlags},
    format::{ClearValue, Format},
    image::{Image, ImageUsage, view::ImageView},
    instance::{Instance, InstanceCreateInfo},
    memory::allocator::StandardMemoryAllocator,
    pipeline::graphics::viewport::Viewport,
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass},
    swapchain::{
        PresentMode, Surface, Swapchain, SwapchainCreateInfo, SwapchainPresentInfo,
        acquire_next_image,
    },
    sync::GpuFuture,
};
use waywin::{
    Waywin, Window,
    event::{WaywinEvent, WindowEvent},
};

fn main() {
    let mut waywin = Waywin::init("imgui-demo").unwrap();
    let mut app = App::new(&mut waywin);
    waywin.run(move |event, running| {
        app.handle_event(event, running);
    });
}

pub struct App {
    device: Arc<Device>,
    queue: Arc<Queue>,
    cmd_alloc: Arc<StandardCommandBufferAllocator>,

    window: Arc<Window>,
    viewport: Viewport,
    swapchain: Arc<Swapchain>,
    framebuffers: Vec<Arc<Framebuffer>>,
    recreate_swapchain: bool,
    render_pass: Arc<RenderPass>,

    imgui: imgui::Context,
    imgui_support: WaywinPlatform,
    imgui_renderer: Renderer,

    future: Option<Box<dyn GpuFuture>>,
}
impl App {
    pub fn new(waywin: &mut Waywin) -> Self {
        let library = VulkanLibrary::new().unwrap();

        let extensions = Surface::required_extensions(waywin).unwrap();
        let instance = Instance::new(
            library,
            InstanceCreateInfo {
                enabled_extensions: extensions,
                ..Default::default()
            },
        )
        .unwrap();

        let (queue_family, physical_device) = instance
            .enumerate_physical_devices()
            .unwrap()
            .filter_map(|pd| {
                pd.queue_family_properties()
                    .iter()
                    .enumerate()
                    .position(|(q, qp)| {
                        qp.queue_flags.intersects(QueueFlags::GRAPHICS)
                            && pd.presentation_support(q as u32, waywin).unwrap()
                    })
                    .map(|q| (q, pd))
            })
            .next()
            .unwrap();

        let (device, mut queues) = Device::new(
            physical_device,
            DeviceCreateInfo {
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index: queue_family as u32,
                    ..Default::default()
                }],
                enabled_extensions: DeviceExtensions {
                    khr_swapchain: true,
                    ..Default::default()
                },
                // enabled_features: todo!(),
                ..Default::default()
            },
        )
        .unwrap();
        let queue = queues.next().unwrap();

        let mem_alloc = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
        let set_alloc = Arc::new(StandardDescriptorSetAllocator::new(
            device.clone(),
            Default::default(),
        ));
        let cmd_alloc = Arc::new(StandardCommandBufferAllocator::new(
            device.clone(),
            Default::default(),
        ));

        let window = Arc::new(waywin.create_window("imgui vulkano demo").unwrap());

        let (w, h) = window.get_physical_size();
        let viewport = Viewport {
            extent: [w as f32, h as f32],
            ..Default::default()
        };

        let format = Format::R8G8B8A8_UNORM;
        let surface = Surface::from_window(instance, window.clone()).unwrap();
        let surface_caps = device
            .physical_device()
            .surface_capabilities(&surface, Default::default())
            .unwrap();
        // let surface_formats = device.physical_device().surface_formats(&surface, Default::default()).unwrap();
        // let surface_present_modes = device.physical_device().surface_present_modes(&surface, Default::default()).unwrap();
        let (swapchain, images) = Swapchain::new(
            device.clone(),
            surface,
            SwapchainCreateInfo {
                min_image_count: surface_caps.min_image_count,
                image_format: format,
                image_extent: window.get_physical_size().into(),
                image_usage: ImageUsage::COLOR_ATTACHMENT,
                present_mode: PresentMode::Fifo,
                ..Default::default()
            },
        )
        .unwrap();

        let render_pass = vulkano::single_pass_renderpass!(
            device.clone(),
            attachments: {
                color: {
                    format: swapchain.image_format(),
                    samples: 1,
                    load_op: Clear,
                    store_op: Store,
                },
            },
            pass: {
                color: [color],
                depth_stencil: {},
            },
        )
        .unwrap();

        let framebuffers = create_framebuffers(&render_pass, images);

        let mut imgui = imgui::Context::create();
        imgui.set_ini_filename(None);
        let imgui_support = WaywinPlatform::new(&mut imgui, &window);
        let subpass = render_pass.clone().first_subpass();
        let mut imgui_renderer = Renderer::new(
            device.clone(),
            mem_alloc.clone(),
            set_alloc.clone(),
            subpass,
        );

        let mut builder = AutoCommandBufferBuilder::primary(
            cmd_alloc.clone(),
            queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();
        imgui_renderer.setup(&mut imgui, &mut builder);
        builder
            .build()
            .unwrap()
            .execute(queue.clone())
            .unwrap()
            .then_signal_fence_and_flush()
            .unwrap()
            .wait(None)
            .unwrap();

        let future = Some(vulkano::sync::now(device.clone()).boxed());
        Self {
            device,
            queue,
            cmd_alloc,

            window,
            viewport,
            swapchain,
            framebuffers,
            recreate_swapchain: false,
            render_pass,

            imgui,
            imgui_support,
            imgui_renderer,
            future,
        }
    }
    pub fn handle_event(&mut self, event: WaywinEvent, running: &mut bool) {
        match &event {
            WaywinEvent::WindowEvent { event, window_id } if *window_id == self.window.id() => {
                match event {
                    WindowEvent::Close => {
                        *running = false;
                    }
                    WindowEvent::Resized => {
                        self.recreate_swapchain = true;
                    }
                    WindowEvent::Paint => {
                        if self.recreate_swapchain {
                            self.recreate_swapchain = false;

                            let (swapchain, images) = self
                                .swapchain
                                .recreate(SwapchainCreateInfo {
                                    image_extent: self.window.get_physical_size().into(),
                                    ..self.swapchain.create_info()
                                })
                                .unwrap();
                            self.swapchain = swapchain;
                            self.framebuffers = create_framebuffers(&self.render_pass, images);

                            let (w, h) = self.window.get_physical_size();
                            self.viewport = Viewport {
                                extent: [w as f32, h as f32],
                                ..Default::default()
                            };
                        }

                        let (image_index, suboptimal, acquire_future) =
                            match acquire_next_image(self.swapchain.clone(), None)
                                .map_err(Validated::unwrap)
                            {
                                Ok(r) => r,
                                Err(VulkanError::OutOfDate) => {
                                    self.recreate_swapchain = true;
                                    return;
                                }
                                Err(e) => panic!("failed to acquire next image: {e}"),
                            };
                        if suboptimal {
                            self.recreate_swapchain = true;
                        }

                        self.imgui_support
                            .prepare_frame(&mut self.imgui, &self.window);
                        let ui = self.imgui.new_frame();
                        ui.show_demo_window(&mut true);
                        self.imgui_support.prepare_render(ui, &self.window);

                        let mut builder = AutoCommandBufferBuilder::primary(
                            self.cmd_alloc.clone(),
                            self.queue.queue_family_index(),
                            CommandBufferUsage::OneTimeSubmit,
                        )
                        .unwrap();
                        builder
                            .begin_render_pass(
                                RenderPassBeginInfo {
                                    render_pass: self.render_pass.clone(),
                                    clear_values: vec![Some(ClearValue::Float([
                                        0.1, 0.2, 0.3, 1.0,
                                    ]))],
                                    ..RenderPassBeginInfo::framebuffer(
                                        self.framebuffers[image_index as usize].clone(),
                                    )
                                },
                                SubpassBeginInfo::default(),
                            )
                            .unwrap()
                            .set_viewport(0, [self.viewport.clone()].into_iter().collect())
                            .unwrap();
                        self.imgui_renderer.render(&mut self.imgui, &mut builder);
                        builder.end_render_pass(Default::default()).unwrap();

                        let future = self
                            .future
                            .take()
                            .unwrap()
                            .join(acquire_future)
                            .then_execute(self.queue.clone(), builder.build().unwrap())
                            .unwrap()
                            .then_swapchain_present(
                                self.queue.clone(),
                                SwapchainPresentInfo::swapchain_image_index(
                                    self.swapchain.clone(),
                                    image_index,
                                ),
                            )
                            .then_signal_fence_and_flush();

                        match future.map_err(Validated::unwrap) {
                            Ok(mut future) => {
                                future.cleanup_finished();
                                self.future = Some(future.boxed());
                            }
                            Err(VulkanError::OutOfDate) => {
                                self.recreate_swapchain = true;
                                self.future = Some(vulkano::sync::now(self.device.clone()).boxed());
                            }
                            Err(e) => {
                                panic!("failed to flush future: {e}");
                            }
                        };
                        self.window.request_redraw();
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        self.imgui_support
            .handle_event(&mut self.imgui, &self.window, event);
    }
}

fn create_framebuffers(
    render_pass: &Arc<RenderPass>,
    images: Vec<Arc<Image>>,
) -> Vec<Arc<Framebuffer>> {
    images
        .into_iter()
        .map(|image| {
            let view = ImageView::new_default(image).unwrap();
            let [w, h, _] = view.image().extent();
            Framebuffer::new(
                render_pass.clone(),
                FramebufferCreateInfo {
                    attachments: vec![view],
                    extent: [w, h],
                    ..Default::default()
                },
            )
            .unwrap()
        })
        .collect()
}
