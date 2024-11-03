use iced::{Center, Fill};
use iced_core::{Element, Theme};
use iced_runtime::{Program, Task};
use iced_wgpu::Renderer;
use iced_widget::{button, column, container};

#[derive(Debug, Clone)]
struct Message {
    s: String,
}
impl Message {
    fn new(s: String) -> Message {
        Message { s: s }
    }
}

struct LuaElement(iced::Element<'static, Message, Theme, Renderer>);
impl Into<iced::Element<'static, Message, Theme, Renderer>> for LuaElement {
    fn into(self) -> iced::Element<'static, Message, Theme, Renderer> {
        self.0
    }
}
impl mlua::UserData for LuaElement {}

struct LuaContainer(container::Container<'static, Message, Theme, Renderer>);
impl LuaContainer {
    fn new(content: impl Into<Element<'static, Message, Theme, Renderer>>) -> LuaContainer {
        LuaContainer(container(content))
    }
}
impl mlua::UserData for LuaContainer {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        // Constructor
        methods.add_meta_function(mlua::MetaMethod::Call, |_, context: LuaElement| {
            Ok(LuaContainer::new(context))
        });
    }
}

macro_rules! impl_fromlua_for {
  ($($typename:ty),*) => {$(
    impl mlua::FromLua for $typename {
      fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        match value {
          mlua::Value::UserData(ud) => Ok(ud.take::<Self>()?),
          _ => unreachable!()
        }
      }
   }
 )*}
}
impl_fromlua_for!(LuaElement, LuaContainer);

#[derive(Debug)]
pub struct ToolkitLua {
    lua: mlua::Lua,
    update: mlua::Function,
    view: mlua::Function,
}

impl ToolkitLua {
    pub fn new() -> ToolkitLua {
        let lua = mlua::Lua::new();
        let globals = lua.globals();
        ToolkitLua {
            lua: lua,
            update: globals.get("update").unwrap(),
            view: globals.get("view").unwrap(),
        }
    }
}

impl Program for ToolkitLua {
    type Theme = Theme;
    type Message = Message;
    type Renderer = Renderer;

    fn update(&mut self, message: Message) -> Task<Message> {
        self.update.call::<()>(()).unwrap();
        Task::none()
    }

    fn view(&self) -> Element<Message, Theme, Renderer> {
        self.view.call::<()>(()).unwrap();
        container(
            container(
                column![
                    button("Load Game"),
                    button("New Game").on_press(Message::new("New Game")),
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
