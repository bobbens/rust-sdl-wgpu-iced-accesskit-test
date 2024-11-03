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
/// Safety:
/// Not safe at _all_. Try to ensure that the base element is `Send`
unsafe impl Send for LuaElement {}
impl From<LuaElement> for iced::Element<'static, Message, Theme, Renderer> {
    fn from(value: LuaElement) -> Self {
        value.0.into()
    }
}
impl mlua::UserData for LuaElement {}

struct LuaButton(iced_widget::Button<'static, Message, Theme, Renderer>);
unsafe impl Send for LuaButton {}
impl From<LuaButton> for iced_widget::Button<'static, Message, Theme, Renderer> {
    fn from(value: LuaButton) -> Self {
        value.0.into()
    }
}
impl mlua::UserData for LuaButton {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(mlua::MetaMethod::ToString, |_, _this, _val: mlua::Value| {
            dbg!("tostring");
            Ok(mlua::Value::Nil)
        });
        methods.add_function_mut("on_press", |_lua, (this, val): (Self, mlua::Value)| {
            dbg!("on_press");
            Ok(LuaButton(this.0.on_press(Message(val))))
        });
    }
}

struct LuaContainer(container::Container<'static, Message, Theme, Renderer>);
unsafe impl Send for LuaContainer {}
impl From<LuaContainer> for iced::Element<'static, Message, Theme, Renderer> {
    fn from(value: LuaContainer) -> Self {
        value.0.into()
    }
}
impl mlua::UserData for LuaContainer {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function_mut("padding", |_lua, (this, padding): (Self, f32)| {
            Ok(LuaContainer(this.0.padding(padding)))
        });
    }
}

macro_rules! impl_fromlua_for {
  ($($typename:ty),*) => {$(
    impl mlua::FromLua for $typename {
      fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        //dbg!(&value);
        match value {
          mlua::Value::UserData(ud) => Ok(ud.take::<Self>()?),
          _ => unreachable!()
        }
      }
   }
 )*}
}
impl_fromlua_for!(LuaButton, LuaElement, LuaContainer, LuaHorizontal, Message);

#[derive(Debug)]
pub struct ToolkitLua {
    lua: mlua::Lua,
    update: mlua::Function,
    view: mlua::Function,
}

impl ToolkitLua {
    pub fn new() -> mlua::Result<ToolkitLua> {
        let lua = mlua::Lua::new();
        open_iced(&lua)?;

        lua.load(
            "function update( msg )
                print( msg )
            end",
        )
        .exec()?;
        lua.load(
            "function view()
                print( iced )
                print( iced.button )
                print( iced.button(\"wtf\") )
                print( iced.button(\"wtf\").__tostring )
                print( iced.button(\"wtf\").on_press )
                print( iced.button(\"wtf\"):on_press(\"Yo, wtf\") )
                return iced.button(\"wtf\"):on_press(\"Yo, wtf\")
            end",
        )
        .exec()?;

        let globals = lua.globals();
        Ok(ToolkitLua {
            lua,
            update: globals.get("update").unwrap(),
            view: globals.get("view").unwrap(),
        })
    }
}

fn value_to_element(
    val: mlua::Value,
) -> mlua::Result<iced::Element<'static, Message, Theme, Renderer>> {
    //dbg!(&val);
    match val {
        mlua::Value::String(s) => Ok(iced_widget::text(s.to_string_lossy()).into()),
        _ => Ok(iced_widget::Space::with_width(0).into()),
    }
}

pub fn open_iced(lua: &mlua::Lua) -> mlua::Result<()> {
    let iced = lua.create_table()?;
    let globals = lua.globals();
    iced.set(
        "container",
        lua.create_function(|_lua, val: mlua::Value| -> mlua::Result<LuaContainer> {
            Ok(LuaContainer(container(value_to_element(val)?).into()))
        })?,
    )?;
    iced.set(
        "button",
        lua.create_function(|_lua, val: mlua::Value| -> mlua::Result<LuaElement> {
            Ok(LuaElement(button(value_to_element(val)?).into()))
        })?,
    )?;
    globals.set("iced", iced)?;
    Ok(())
}

impl Program for ToolkitLua {
    type Theme = Theme;
    type Message = Message;
    type Renderer = Renderer;

    fn update(&mut self, message: Message) -> Task<Message> {
        self.update.call::<()>(message.0).unwrap();
        Task::none()
    }

    fn view(&self) -> Element<Message, Theme, Renderer> {
        let ele = value_to_element(self.view.call::<mlua::Value>(()).unwrap_or_else(|err| {
            panic!("{}", err);
        }))
        .unwrap();
        container(
            container(
                column![
                    button("Load Game"),
                    button("New Game").on_press(Message::new(mlua::Value::Nil)),
                    ele,
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
