use sdl2::event::{Event, WindowEvent};

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
        Keycode::A => Character(SmolStr::new("a")),
        Keycode::BACKSPACE => Named(NamedKey::Backspace),
        Keycode::TAB => Named(NamedKey::Tab),
        Keycode::RETURN => Named(NamedKey::Enter),
        Keycode::ESCAPE => Named(NamedKey::Escape),
        Keycode::SPACE => Named(NamedKey::Space),
        _ => iced::keyboard::Key::Unidentified,
    }
}

pub fn physical_key(scancode: &sdl2::keyboard::Scancode) -> iced::keyboard::key::Physical {
    use iced_core::keyboard::key::{Code, NativeCode, Physical};
    use sdl2::keyboard::Scancode;

    match scancode {
        &Scancode::A => Physical::Code(Code::KeyA),
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
