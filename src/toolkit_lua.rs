use iced::{Center, Fill};
use iced_core::{Element, Theme};
use iced_runtime::{Program, Task};
use iced_wgpu::Renderer;
use iced_widget::{button, column, container};
use mlua::{Chunk, FromLuaMulti, UserData};

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
/// Safety:
/// Not safe at _all_. Try to ensure that the base element is `Send`
unsafe impl Send for LuaElement {}
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
unsafe impl Send for LuaContainer {}
impl LuaContainer {
    fn new<C: Into<Element<'static, Message, Theme, Renderer>> + Send>(content: C) -> LuaContainer {
        LuaContainer(container(content))
    }
}
unsafe impl Send for LuaContainer {}
impl mlua::UserData for LuaContainer {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function_mut("padding", |_lua, (this, padding): (Self, f32)| {
            Ok(LuaContainer(this.0.padding(padding)))
        });
        methods.add_function_mut("dbg_padding", |_lua, (this, padding): (Self, f32)| {
            dbg!(padding);
            Ok(LuaContainer(this.0.padding(padding)))
        });
    }
}

impl mlua::FromLuaMulti for LuaElement {
    fn from_lua_multi(values: mlua::MultiValue, lua: &mlua::Lua) -> mlua::Result<Self> {
        Ok(match values.len() {
            0 => LuaElement(iced_widget::Space::with_width(0).into()),
            1 => match &values[0] {
                mlua::Value::UserData(any_user_data) => any_user_data.take::<Self>()?,
                mlua::Value::String(s) => {
                    LuaElement(iced_widget::text(s.to_str().unwrap().to_string()).into())
                }
                _ => todo!(),
            },
            _ => todo!(),
        })
    }
}

macro_rules! impl_fromlua_for {
  ($($typename:ty),*) => {$(
    impl mlua::FromLua for $typename {
      fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
          dbg!(&value);
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

        let element = lua
            .create_function(|lua, widget: mlua::MultiValue| {
                LuaElement::from_lua_multi(widget, lua)
            })
            .unwrap();
        lua.globals().set("Element", element).unwrap();
        let container = lua
            .create_function(|_, element: LuaElement| Ok(LuaContainer::new(element)))
            .unwrap();
        lua.globals().set("Container", container).unwrap();

        lua.load("function update() end").exec().unwrap();
        lua.load(
            "function view()
                local element = Element(\"wtf\")
                local container = Container(Element(\"Hi world\")):padding(2.0):dbg_padding(1.0)
            end",
        )
        .exec()
        .unwrap();

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
