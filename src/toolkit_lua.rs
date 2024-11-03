use iced::{Center, Fill};
use iced_core::{Element, Theme};
use iced_runtime::{Program, Task};
use iced_wgpu::Renderer;
use iced_widget::{button, column, container};

#[derive(Debug, Clone)]
pub struct Message(mlua::Value);
impl Message {
    fn new<V: Into<mlua::Value> + Send>(value: V) -> Message {
        Message(value.into())
    }
}
impl mlua::UserData for Message {}

// Wraper for Horizontal
struct LuaHorizontal(iced::alignment::Horizontal);
impl Into<iced::alignment::Horizontal> for LuaHorizontal {
    fn into(self) -> iced::alignment::Horizontal {
        self.0
    }
}
impl mlua::UserData for LuaHorizontal {}

struct LuaElement(iced::Element<'static, Message, Theme, Renderer>);
impl Into<iced::Element<'static, Message, Theme, Renderer>> for LuaElement {
    fn into(self) -> iced::Element<'static, Message, Theme, Renderer> {
        self.0
    }
}
impl mlua::UserData for LuaElement {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {}
}
// TODO fix this
unsafe impl Send for LuaElement {}

struct LuaContainer(container::Container<'static, Message, Theme, Renderer>);
impl LuaContainer {
    //fn new(content: impl Into<Element<'static, Message, Theme, Renderer>>) -> LuaContainer {
    fn new<C: Into<Element<'static, Message, Theme, Renderer>> + Send>(content: C) -> LuaContainer {
        LuaContainer(container(content))
    }
}
unsafe impl Send for LuaContainer {}
impl mlua::UserData for LuaContainer {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("add", |_, this, value: LuaHorizontal| {
            Ok(LuaContainer(this.0.align_x(value)))
            //Ok(())
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
impl_fromlua_for!(LuaElement, LuaContainer, LuaHorizontal, Message);

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
            lua,
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
                    button("New Game").on_press(Message::new(mlua::Value::Nil)),
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
