use imgui::{Context, Ui};
use waywin::{
    Window,
    event::{Event, Key, LogicalKey, PointerButton, ScrollDirection},
};

pub struct WaywinPlatform {}
impl WaywinPlatform {
    pub fn new(imgui: &mut Context, window: &Window) -> Self {
        let io = imgui.io_mut();
        // io.backend_flags.insert(BackendFlags::HAS_MOUSE_CURSORS);
        // io.backend_flags.insert(BackendFlags::HAS_SET_MOUSE_POS);
        let size = window.get_physical_size();
        io.display_size = [size.0 as f32, size.1 as f32];
        imgui.set_platform_name(format!(
            "imgui-waywin-support {}",
            env!("CARGO_PKG_VERSION")
        ));

        Self {}
    }

    pub fn handle_event(&mut self, imgui: &mut Context, window: &Window, event: Event) {
        let io = imgui.io_mut();
        match event {
            Event::Resized => {
                let (w, h) = window.get_logical_size();
                io.display_size = [w as f32, h as f32];
            }
            Event::NewScaleFactor => {
                let scale = window.get_scale() as f32;
                io.display_framebuffer_scale = [scale, scale];
            }
            Event::Focus(focus) => {
                if !focus {
                    io.app_focus_lost = true;
                }
            }
            Event::Key {
                down,
                physical_key: _,
                logical_key: _,
                text,
                text_raw: _,
                logical_key_unmodified,
            } => {
                for char in text.chars() {
                    io.add_input_character(char);
                }

                match &logical_key_unmodified {
                    LogicalKey::Key(Key::LCtrl)
                    | LogicalKey::Key(Key::RCtrl)
                    | LogicalKey::Key(Key::Ctrl) => io.add_key_event(imgui::Key::ModCtrl, down),

                    LogicalKey::Key(Key::LShift)
                    | LogicalKey::Key(Key::RShift)
                    | LogicalKey::Key(Key::Shift) => io.add_key_event(imgui::Key::ModShift, down),

                    LogicalKey::Key(Key::LAlt)
                    | LogicalKey::Key(Key::RAlt)
                    | LogicalKey::Key(Key::Alt) => io.add_key_event(imgui::Key::ModAlt, down),

                    LogicalKey::Key(Key::LSuper)
                    | LogicalKey::Key(Key::RSuper)
                    | LogicalKey::Key(Key::Super) => io.add_key_event(imgui::Key::ModSuper, down),

                    _ => {}
                }

                let key = match logical_key_unmodified.as_ref() {
                    LogicalKey::Key(Key::Tab) => imgui::Key::Tab,
                    LogicalKey::Key(Key::LeftArrow) => imgui::Key::LeftArrow,
                    LogicalKey::Key(Key::RightArrow) => imgui::Key::RightArrow,
                    LogicalKey::Key(Key::UpArrow) => imgui::Key::UpArrow,
                    LogicalKey::Key(Key::DownArrow) => imgui::Key::DownArrow,
                    LogicalKey::Key(Key::PageUp) => imgui::Key::PageUp,
                    LogicalKey::Key(Key::PageDown) => imgui::Key::PageDown,
                    LogicalKey::Key(Key::Home) => imgui::Key::Home,
                    LogicalKey::Key(Key::End) => imgui::Key::End,
                    LogicalKey::Key(Key::Insert) => imgui::Key::Insert,
                    LogicalKey::Key(Key::Delete) => imgui::Key::Delete,
                    LogicalKey::Key(Key::Backspace) => imgui::Key::Backspace,
                    LogicalKey::Key(Key::Space) => imgui::Key::Space,
                    LogicalKey::Key(Key::Enter) => imgui::Key::Enter,
                    LogicalKey::Key(Key::Escape) => imgui::Key::Escape,
                    LogicalKey::Key(Key::LCtrl) => imgui::Key::LeftCtrl,
                    LogicalKey::Key(Key::LShift) => imgui::Key::LeftShift,
                    LogicalKey::Key(Key::LAlt) => imgui::Key::LeftAlt,
                    LogicalKey::Key(Key::LSuper) => imgui::Key::LeftSuper,
                    LogicalKey::Key(Key::RCtrl) => imgui::Key::RightCtrl,
                    LogicalKey::Key(Key::RShift) => imgui::Key::RightShift,
                    LogicalKey::Key(Key::RAlt) => imgui::Key::RightAlt,
                    LogicalKey::Key(Key::RSuper) => imgui::Key::RightSuper,
                    LogicalKey::Key(Key::Menu) => imgui::Key::Menu,
                    LogicalKey::Key(Key::Key0) => imgui::Key::Alpha0,
                    LogicalKey::Key(Key::Key1) => imgui::Key::Alpha1,
                    LogicalKey::Key(Key::Key2) => imgui::Key::Alpha2,
                    LogicalKey::Key(Key::Key3) => imgui::Key::Alpha3,
                    LogicalKey::Key(Key::Key4) => imgui::Key::Alpha4,
                    LogicalKey::Key(Key::Key5) => imgui::Key::Alpha5,
                    LogicalKey::Key(Key::Key6) => imgui::Key::Alpha6,
                    LogicalKey::Key(Key::Key7) => imgui::Key::Alpha7,
                    LogicalKey::Key(Key::Key8) => imgui::Key::Alpha8,
                    LogicalKey::Key(Key::Key9) => imgui::Key::Alpha9,
                    LogicalKey::Character("a") => imgui::Key::A,
                    LogicalKey::Character("b") => imgui::Key::B,
                    LogicalKey::Character("c") => imgui::Key::C,
                    LogicalKey::Character("d") => imgui::Key::D,
                    LogicalKey::Character("e") => imgui::Key::E,
                    LogicalKey::Character("f") => imgui::Key::F,
                    LogicalKey::Character("g") => imgui::Key::G,
                    LogicalKey::Character("h") => imgui::Key::H,
                    LogicalKey::Character("i") => imgui::Key::I,
                    LogicalKey::Character("j") => imgui::Key::J,
                    LogicalKey::Character("k") => imgui::Key::K,
                    LogicalKey::Character("l") => imgui::Key::L,
                    LogicalKey::Character("m") => imgui::Key::M,
                    LogicalKey::Character("n") => imgui::Key::N,
                    LogicalKey::Character("o") => imgui::Key::O,
                    LogicalKey::Character("p") => imgui::Key::P,
                    LogicalKey::Character("q") => imgui::Key::Q,
                    LogicalKey::Character("r") => imgui::Key::R,
                    LogicalKey::Character("s") => imgui::Key::S,
                    LogicalKey::Character("t") => imgui::Key::T,
                    LogicalKey::Character("u") => imgui::Key::U,
                    LogicalKey::Character("v") => imgui::Key::V,
                    LogicalKey::Character("w") => imgui::Key::W,
                    LogicalKey::Character("x") => imgui::Key::X,
                    LogicalKey::Character("y") => imgui::Key::Y,
                    LogicalKey::Character("z") => imgui::Key::Z,
                    LogicalKey::Key(Key::F1) => imgui::Key::F1,
                    LogicalKey::Key(Key::F2) => imgui::Key::F2,
                    LogicalKey::Key(Key::F3) => imgui::Key::F3,
                    LogicalKey::Key(Key::F4) => imgui::Key::F4,
                    LogicalKey::Key(Key::F5) => imgui::Key::F5,
                    LogicalKey::Key(Key::F6) => imgui::Key::F6,
                    LogicalKey::Key(Key::F7) => imgui::Key::F7,
                    LogicalKey::Key(Key::F8) => imgui::Key::F8,
                    LogicalKey::Key(Key::F9) => imgui::Key::F9,
                    LogicalKey::Key(Key::F10) => imgui::Key::F10,
                    LogicalKey::Key(Key::F11) => imgui::Key::F11,
                    LogicalKey::Key(Key::F12) => imgui::Key::F12,
                    LogicalKey::Character("'") => imgui::Key::Apostrophe,
                    LogicalKey::Character(",") => imgui::Key::Comma,
                    LogicalKey::Key(Key::Minus) => imgui::Key::Minus,
                    LogicalKey::Key(Key::Period) | LogicalKey::Key(Key::NumpadDelete) => {
                        imgui::Key::Period
                    }
                    LogicalKey::Key(Key::Slash) => imgui::Key::Slash,
                    LogicalKey::Character(";") => imgui::Key::Semicolon,
                    LogicalKey::Character("=") => imgui::Key::Equal,
                    LogicalKey::Character("[") => imgui::Key::LeftBracket,
                    LogicalKey::Character("\\") => imgui::Key::Backslash,
                    LogicalKey::Character("]") => imgui::Key::RightBracket,
                    // LogicalKey::Key(Key::Tab) => imgui::Key::GraveAccent,
                    LogicalKey::Key(Key::CapsLock) => imgui::Key::CapsLock,
                    LogicalKey::Key(Key::ScrollLock) => imgui::Key::ScrollLock,
                    LogicalKey::Key(Key::NumLock) => imgui::Key::NumLock,
                    LogicalKey::Key(Key::PrintScreen) => imgui::Key::PrintScreen,
                    LogicalKey::Key(Key::Pause) => imgui::Key::Pause,
                    LogicalKey::Key(Key::NumpadInsert) => imgui::Key::Keypad0,
                    LogicalKey::Key(Key::NumpadEnd) => imgui::Key::Keypad1,
                    LogicalKey::Key(Key::NumpadDownArrow) => imgui::Key::Keypad2,
                    LogicalKey::Key(Key::NumpadPageDown) => imgui::Key::Keypad3,
                    LogicalKey::Key(Key::NumpadLeftArrow) => imgui::Key::Keypad4,
                    LogicalKey::Key(Key::NumpadBegin) => imgui::Key::Keypad5,
                    LogicalKey::Key(Key::NumpadRightArrow) => imgui::Key::Keypad6,
                    LogicalKey::Key(Key::NumpadHome) => imgui::Key::Keypad7,
                    LogicalKey::Key(Key::NumpadUpArrow) => imgui::Key::Keypad8,
                    LogicalKey::Key(Key::NumpadPageUp) => imgui::Key::Keypad9,
                    LogicalKey::Key(Key::NumpadDecimal) => imgui::Key::KeypadDecimal,
                    LogicalKey::Key(Key::NumpadDivide) => imgui::Key::KeypadDivide,
                    LogicalKey::Key(Key::NumpadMultiply) => imgui::Key::KeypadMultiply,
                    LogicalKey::Key(Key::NumpadSubtract) => imgui::Key::KeypadSubtract,
                    LogicalKey::Key(Key::NumpadAdd) => imgui::Key::KeypadAdd,
                    LogicalKey::Key(Key::NumpadEnter) => imgui::Key::KeypadEnter,
                    // LogicalKey::Key(Key::Tab) => imgui::Key::KeypadEqual,
                    // LogicalKey::Key(Key::Tab) => imgui::Key::ModShortcut,
                    _ => return,
                };

                io.add_key_event(key, down);
            }
            Event::PointerMoved(x, y) => {
                io.add_mouse_pos_event([x as f32, y as f32]);
            }
            Event::PointerButton { down, button } => {
                let button = match button {
                    PointerButton::Left => imgui::MouseButton::Left,
                    PointerButton::Right => imgui::MouseButton::Right,
                    PointerButton::Middle => imgui::MouseButton::Middle,
                    PointerButton::Forward => imgui::MouseButton::Extra2,
                    PointerButton::Back => imgui::MouseButton::Extra1,
                    PointerButton::Unknown(_) => return,
                };
                io.add_mouse_button_event(button, down);
            }
            Event::Scroll { direction, value } => {
                let value = value as f32 / 26.0;
                let wheel = match direction {
                    ScrollDirection::Vertical => [0.0, value],
                    ScrollDirection::Horizontal => [value, 0.0],
                };
                io.add_mouse_wheel_event(wheel);
            }
            _ => {}
        }
    }

    pub fn prepare_frame(&mut self, _imgui: &mut Context, _window: &Window) {}

    pub fn prepare_render(&mut self, _ui: &Ui, _window: &Window) {}
}
