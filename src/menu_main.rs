use crate::toolkit;
use crate::toolkit::Message as MessageBase;
use iced::{Center, Fill};
use iced_core::{Element, Theme};
use iced_wgpu::Renderer;
use iced_widget::{button, column, container};

#[derive(PartialEq, Eq)]
pub struct MenuMain {}

#[derive(Debug, Clone)]
pub enum Message {
    None,
    LoadGame,
    NewGame,
    Editors,
    Options,
    Credits,
    ExitGame,
}

impl MenuMain {
    pub fn new() -> MenuMain {
        MenuMain {}
    }
}

fn pilot_new(accept: bool, name: String) -> MessageBase {
    match accept {
        true => MessageBase::OpenDialogueOK(
            format!("Your name is {name}!", name = name),
            &|| -> MessageBase { MessageBase::CloseWindows(2) },
        ),
        false => MessageBase::CloseWindow,
    }
}

impl toolkit::Window for MenuMain {
    fn update(&mut self, message: MessageBase) -> MessageBase {
        if let MessageBase::MenuMain(m) = message {
            match m {
                Message::NewGame => MessageBase::OpenDialogueInput(
                    String::from("What will you name your new pilot?"),
                    &pilot_new,
                ),
                Message::ExitGame => {
                    crate::quit();
                    MessageBase::CloseWindow
                },
                Message::Options => MessageBase::OpenDialogueOK(
                    String::from("Not implemented yet!"),
                    &toolkit::dialogue_noop_ok,
                ),
                _ => MessageBase::None,
            }
        } else {
            MessageBase::None
        }
    }

    fn view(&self) -> Element<MessageBase, Theme, Renderer> {
        container(
            container(
                column![
                    button("Load Game"),
                    button("New Game").on_press(MessageBase::MenuMain(Message::NewGame)),
                    button("Editors").on_press(MessageBase::MenuMain(Message::Editors)),
                    button("Options").on_press(MessageBase::MenuMain(Message::Options)),
                    //button("Credits").on_press(Message::Credits),
                    button("Exit Game").on_press(MessageBase::MenuMain(Message::ExitGame)),
                ]
                .spacing(10)
                .padding(20)
                .align_x(Center),
            )
            .style(crate::toolkit::window)
            .align_x(Center)
            .width(150),
        )
        .style(container::transparent)
        .center(Fill)
        .into()
    }
}
