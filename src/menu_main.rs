use crate::toolkit;
use crate::toolkit::Message as MessageBase;
use iced::{Center, Fill};
use iced_core::{Element, Theme};
use iced_runtime::{Program, Task};
use iced_wgpu::Renderer;
use iced_widget::{button, column, container};

pub struct MenuMain {
    pub state: Message,
}

#[derive(Debug, Clone)]
pub enum Message {
    None,
    //LoadGame,
    NewGame,
    Editors,
    Options,
    Credits,
    ExitGame,
}

impl MenuMain {
    pub fn new() -> MenuMain {
        MenuMain {
            state: Message::None,
        }
    }
}

impl toolkit::Window for MenuMain {
    fn update(&mut self, message: MessageBase) -> Task<MessageBase> {
        match message {
            MessageBase::MenuMain(m) => {
                self.state = m;
                //self.program.open( toolkit_lua::ToolkitWindow::Lua( toolkit_lua::ToolkitWindowLua::new().unwrap_or_else(|err| {
                //    panic!("{}", err);
                //})));
            }
            _ => unreachable!(),
        };
        Task::none()
    }

    fn view(&self) -> Element<MessageBase, Theme, Renderer> {
        container(
            container(
                column![
                    button("Load Game"),
                    button("New Game").on_press(MessageBase::MenuMain(Message::NewGame)),
                    //button("Editors").on_press(Message::Editors),
                    //button("Options").on_press(Message::Options),
                    //button("Credits").on_press(Message::Credits),
                    //button("Exit Game").on_press(Message::ExitGame),
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
