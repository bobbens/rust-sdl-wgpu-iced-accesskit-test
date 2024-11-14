use sdl2::event::{Event, WindowEvent};

pub struct Clipboard(sdl2::clipboard::ClipboardUtil);
impl Clipboard {
    pub fn new(clipboard: sdl2::clipboard::ClipboardUtil) -> Self {
        Self(clipboard)
    }
}
impl iced_core::Clipboard for Clipboard {
    fn read(&self, kind: iced_core::clipboard::Kind) -> Option<String> {
        match kind {
            iced_core::clipboard::Kind::Standard => match self.0.has_clipboard_text() {
                true => Some(self.0.clipboard_text().unwrap()),
                false => None,
            },
            iced_core::clipboard::Kind::Primary => match self.0.has_primary_selection_text() {
                true => Some(self.0.primary_selection_text().unwrap()),
                false => None,
            },
        }
    }
    fn write(&mut self, _kind: iced_core::clipboard::Kind, contents: String) {
        self.0.set_clipboard_text(contents.as_str()).unwrap();
    }
}

pub fn mouse_button(mouse_btn: &sdl2::mouse::MouseButton) -> iced::mouse::Button {
    match mouse_btn {
        sdl2::mouse::MouseButton::Left => iced::mouse::Button::Left,
        sdl2::mouse::MouseButton::Right => iced::mouse::Button::Right,
        sdl2::mouse::MouseButton::Middle => iced::mouse::Button::Middle,
        sdl2::mouse::MouseButton::X1 => iced::mouse::Button::Back,
        sdl2::mouse::MouseButton::X2 => iced::mouse::Button::Forward,
        sdl2::mouse::MouseButton::Unknown => iced::mouse::Button::Other(0),
    }
}

pub fn key(keycode: &sdl2::keyboard::Keycode) -> iced::keyboard::key::Key {
    use iced::keyboard::key::Key::{Character, Named};
    use iced::keyboard::key::Named as NamedKey;
    use iced_core::SmolStr;
    use sdl2::keyboard::Keycode;

    match *keycode {
        Keycode::RETURN => Named(NamedKey::Enter),
        Keycode::ESCAPE => Named(NamedKey::Escape),
        Keycode::BACKSPACE => Named(NamedKey::Backspace),
        Keycode::TAB => Named(NamedKey::Tab),
        Keycode::SPACE => Named(NamedKey::Space),
        //Keycode::EXCLAIM => Named(NamedKey::Exclaim),
        //Keycode::QUOTEDBL => Named(NamedKey::Tab),
        //Keycode::HASH => Named(NamedKey::Tab),
        //Keycode::DOLLAR => Named(NamedKey::Tab),
        //Keycode::PERCENT => Named(NamedKey::Tab),
        //Keycode::AMPERSAND => Named(NamedKey::Tab),
        //Keycode::QUOTE => Named(NamedKey::Tab),
        //Keycode::LEFTPAREN => Named(NamedKey::Tab),
        //Keycode::RIGHTPAREN => Named(NamedKey::Tab),
        //Keycode::ASTERISK => Named(NamedKey::Tab),
        //Keycode::PLUS => Named(NamedKey::Tab),
        //Keycode::COMMA => Named(NamedKey::Tab),
        //Keycode::MINUS => Named(NamedKey::Tab),
        //Keycode::PERIOD => Named(NamedKey::Tab),
        //Keycode::SLASH => Named(NamedKey::Tab),
        Keycode::NUM_0 => Character(SmolStr::new("0")),
        Keycode::NUM_1 => Character(SmolStr::new("1")),
        Keycode::NUM_2 => Character(SmolStr::new("2")),
        Keycode::NUM_3 => Character(SmolStr::new("3")),
        Keycode::NUM_4 => Character(SmolStr::new("4")),
        Keycode::NUM_5 => Character(SmolStr::new("5")),
        Keycode::NUM_6 => Character(SmolStr::new("6")),
        Keycode::NUM_7 => Character(SmolStr::new("7")),
        Keycode::NUM_8 => Character(SmolStr::new("8")),
        Keycode::NUM_9 => Character(SmolStr::new("9")),
        //Keycode:: => Named(NamedKey::Tab),
        Keycode::A => Character(SmolStr::new("a")),
        Keycode::B => Character(SmolStr::new("b")),
        Keycode::C => Character(SmolStr::new("c")),
        Keycode::D => Character(SmolStr::new("d")),
        Keycode::E => Character(SmolStr::new("e")),
        Keycode::F => Character(SmolStr::new("f")),
        Keycode::G => Character(SmolStr::new("g")),
        Keycode::H => Character(SmolStr::new("h")),
        Keycode::I => Character(SmolStr::new("i")),
        Keycode::J => Character(SmolStr::new("j")),
        Keycode::K => Character(SmolStr::new("k")),
        Keycode::L => Character(SmolStr::new("l")),
        Keycode::M => Character(SmolStr::new("m")),
        Keycode::N => Character(SmolStr::new("n")),
        Keycode::O => Character(SmolStr::new("o")),
        Keycode::P => Character(SmolStr::new("p")),
        Keycode::Q => Character(SmolStr::new("q")),
        Keycode::R => Character(SmolStr::new("r")),
        Keycode::S => Character(SmolStr::new("s")),
        Keycode::T => Character(SmolStr::new("t")),
        Keycode::U => Character(SmolStr::new("u")),
        Keycode::V => Character(SmolStr::new("v")),
        Keycode::W => Character(SmolStr::new("w")),
        Keycode::X => Character(SmolStr::new("x")),
        Keycode::Y => Character(SmolStr::new("y")),
        Keycode::Z => Character(SmolStr::new("z")),
        _ => iced::keyboard::Key::Unidentified,
    }
}

pub fn physical_key(scancode: &sdl2::keyboard::Scancode) -> iced::keyboard::key::Physical {
    use iced_core::keyboard::key::{Code, NativeCode, Physical};
    use sdl2::keyboard::Scancode;

    match &scancode {
        Scancode::A => Physical::Code(Code::KeyA),
        _ => Physical::Unidentified(NativeCode::Unidentified),
    }
}

pub fn modifier(keymod: &sdl2::keyboard::Mod) -> iced::keyboard::Modifiers {
    use iced::keyboard::Modifiers;
    use sdl2::keyboard::Mod;
    let mut modifiers = iced_core::keyboard::Modifiers::empty();
    if keymod.contains(Mod::LSHIFTMOD | Mod::RSHIFTMOD) {
        modifiers.insert(Modifiers::SHIFT);
    }
    if keymod.contains(Mod::LCTRLMOD | Mod::RCTRLMOD) {
        modifiers.insert(Modifiers::CTRL);
    }
    if keymod.contains(Mod::LALTMOD | Mod::RALTMOD) {
        modifiers.insert(Modifiers::ALT);
    }
    if keymod.contains(Mod::LGUIMOD | Mod::RGUIMOD) {
        modifiers.insert(Modifiers::LOGO);
    }
    modifiers
}

pub fn window_event(event: &Event, scale_factor: f64) -> Option<iced_core::Event> {
    match event {
        Event::Window {
            //window_id,
            win_event: WindowEvent::SizeChanged(width, height),
            ..
        } => Some(iced_core::Event::Window(iced_core::window::Event::Resized(
            iced::Size {
                width: *width as f32,
                height: *height as f32,
            },
        ))),
        Event::Window {
            win_event: WindowEvent::Enter,
            ..
        } => Some(iced_core::Event::Mouse(
            iced_core::mouse::Event::CursorEntered,
        )),
        Event::Window {
            win_event: WindowEvent::Leave,
            ..
        } => Some(iced_core::Event::Mouse(iced_core::mouse::Event::CursorLeft)),
        Event::Window {
            win_event: WindowEvent::FocusGained,
            ..
        } => Some(iced_core::Event::Window(iced_core::window::Event::Focused)),
        Event::Window {
            win_event: WindowEvent::FocusLost,
            ..
        } => Some(iced_core::Event::Window(
            iced_core::window::Event::Unfocused,
        )),
        Event::MouseMotion { x, y, .. } => {
            let s = 1.0 / scale_factor as f32;
            let fx = (*x as f32) * s;
            let fy = (*y as f32) * s;
            Some(iced_core::Event::Mouse(
                iced_core::mouse::Event::CursorMoved {
                    position: iced_core::Point::new(fx, fy),
                },
            ))
        }
        Event::MouseButtonDown { mouse_btn, .. } => {
            let btn = mouse_button(mouse_btn);
            Some(iced_core::Event::Mouse(
                iced_core::mouse::Event::ButtonPressed(btn),
            ))
        }
        Event::MouseButtonUp { mouse_btn, .. } => {
            let btn = mouse_button(mouse_btn);
            Some(iced_core::Event::Mouse(
                iced_core::mouse::Event::ButtonReleased(btn),
            ))
        }
        Event::TextInput { text, .. } => Some(iced_core::Event::Keyboard(
            iced_core::keyboard::Event::KeyPressed {
                key: iced::keyboard::Key::Unidentified,
                modified_key: iced::keyboard::Key::Unidentified,
                physical_key: iced_core::keyboard::key::Physical::Unidentified(
                    iced_core::keyboard::key::NativeCode::Unidentified,
                ),
                location: iced_core::keyboard::Location::Standard,
                modifiers: iced_core::keyboard::Modifiers::empty(),
                text: Some(iced_core::SmolStr::new(text)),
            },
        )),
        Event::KeyDown {
            keycode,
            scancode,
            keymod,
            ..
        } => {
            let k = match keycode {
                Some(c) => key(c),
                None => iced::keyboard::Key::Unidentified,
            };
            Some(iced_core::Event::Keyboard(
                iced_core::keyboard::Event::KeyPressed {
                    key: k.clone(),
                    modified_key: k,
                    physical_key: match scancode {
                        Some(c) => physical_key(c),
                        None => iced_core::keyboard::key::Physical::Unidentified(
                            iced_core::keyboard::key::NativeCode::Unidentified,
                        ),
                    },
                    location: iced_core::keyboard::Location::Standard,
                    modifiers: modifier(keymod),
                    text: None,
                },
            ))
        }
        Event::KeyUp {
            keycode,
            scancode,
            keymod,
            ..
        } => {
            let k = match keycode {
                Some(c) => key(c),
                None => iced::keyboard::Key::Unidentified,
            };
            Some(iced_core::Event::Keyboard(
                iced_core::keyboard::Event::KeyReleased {
                    key: k.clone(),
                    modified_key: k,
                    physical_key: match scancode {
                        Some(c) => physical_key(c),
                        None => iced_core::keyboard::key::Physical::Unidentified(
                            iced_core::keyboard::key::NativeCode::Unidentified,
                        ),
                    },
                    location: iced_core::keyboard::Location::Standard,
                    modifiers: modifier(keymod),
                },
            ))
        }
        Event::Quit { .. } => Some(iced_core::Event::Window(
            iced_core::window::Event::CloseRequested,
        )),
        _ => None,
    }
}
