use iced::{Center, Fill};
use iced_core::{Element, Theme};
use iced_runtime::{Program, Task};
use iced_wgpu::Renderer;
use iced_widget::{button, column, container};

macro_rules! lua_wrapper_min {
    ($wrapper: ident, $wrapped: ty) => {
        struct $wrapper($wrapped);
        unsafe impl Send for $wrapper {}
        impl From<$wrapper> for $wrapped {
            fn from(value: $wrapper) -> Self {
                value.0.into()
            }
        }
    };
}

macro_rules! lua_wrapper {
    ($wrapper: ident, $wrapped: ty) => {
        lua_wrapper_min!($wrapper, $wrapped);
        impl mlua::FromLua for $wrapper {
            fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
                match value {
                    mlua::Value::UserData(ud) => Ok(ud.take::<Self>()?),
                    _ => Err(mlua::Error::FromLuaConversionError {
                        from: value.type_name(),
                        to: String::from("$wrapper"),
                        message: None,
                    }),
                }
            }
        }
    };
}

macro_rules! impl_element_for {
    ($($typename:ty),*) => {$(
        impl From<$typename> for iced::Element<'static, Message, Theme, Renderer> {
            fn from(value: $typename) -> Self {
                value.0.into()
            }
        }
    )*}
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

#[derive(Debug, Clone)]
pub struct Message(mlua::Value);
impl Message {
    fn new<V: Into<mlua::Value> + Send>(value: V) -> Message {
        Message(value.into())
    }
}
impl mlua::UserData for Message {}
impl_fromlua_for!(Message);

// Wraper for Length
lua_wrapper!(LuaLength, iced::Length);
impl mlua::UserData for LuaLength {}

// Wraper for Padding
lua_wrapper_min!(LuaPadding, iced::Padding);
impl mlua::UserData for LuaPadding {}
impl mlua::FromLua for LuaPadding {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        match value {
            mlua::Value::Integer(n) => Ok(LuaPadding(iced::Padding::new(n as f32))),
            mlua::Value::Number(n) => Ok(LuaPadding(iced::Padding::new(n as f32))),
            mlua::Value::UserData(ud) => Ok(ud.take::<Self>()?),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: String::from("LuaPadding"),
                message: None,
            }),
        }
    }
}

// Wraper for Horizontal
lua_wrapper!(LuaHorizontal, iced::alignment::Horizontal);
impl mlua::UserData for LuaHorizontal {}

// Element Wrapper
lua_wrapper!(LuaElement, iced::Element<'static, Message, Theme, Renderer>);
impl mlua::UserData for LuaElement {}
impl_element_for!(LuaButton, LuaContainer, LuaColumn);

// Button Wrapper
lua_wrapper!(
    LuaButton,
    iced_widget::Button<'static, Message, Theme, Renderer>
);
impl mlua::UserData for LuaButton {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function_mut("on_press", |_lua, (this, val): (Self, mlua::Value)| {
            Ok(LuaButton(this.0.on_press(Message(val))))
        });
        methods.add_function_mut("width", |_lua, (this, val): (Self, LuaLength)| {
            Ok(LuaButton(this.0.width(val)))
        });
        methods.add_function_mut("height", |_lua, (this, val): (Self, LuaLength)| {
            Ok(LuaButton(this.0.height(val)))
        });
        methods.add_function_mut("padding", |_lua, (this, val): (Self, LuaPadding)| {
            Ok(LuaButton(this.0.padding(val)))
        });
        methods.add_function_mut("clip", |_lua, (this, val): (Self, mlua::Value)| {
            Ok(LuaButton(this.0.clip(val.as_boolean().unwrap_or(false))))
        });
    }
}

// Column Wrapper
lua_wrapper!(
    LuaColumn,
    iced_widget::Column<'static, Message, Theme, Renderer>
);
impl mlua::UserData for LuaColumn {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function_mut("padding", |_lua, (this, padding): (Self, f32)| {
            Ok(LuaColumn(this.0.padding(padding)))
        });
    }
}

// Container Wrapper
lua_wrapper!(
    LuaContainer,
    iced_widget::Container<'static, Message, Theme, Renderer>
);
impl mlua::UserData for LuaContainer {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function_mut("padding", |_lua, (this, padding): (Self, f32)| {
            Ok(LuaContainer(this.0.padding(padding)))
        });
    }
}

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
                return iced.column{
                        iced.button('Load Game'),
                        iced.button('New Game')
                            :on_press('New Game'),
                        iced.button('wtf')
                            :on_press('Yo, wtf')
                            :padding(10)
                    }
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
        mlua::Value::UserData(ud) => {
            if ud.is::<LuaButton>() {
                Ok(ud.take::<LuaButton>()?.into())
            } else if ud.is::<LuaColumn>() {
                Ok(ud.take::<LuaColumn>()?.into())
            } else if ud.is::<LuaContainer>() {
                Ok(ud.take::<LuaContainer>()?.into())
            } else {
                Err(mlua::Error::UserDataTypeMismatch)
            }
        }
        _ => Err(mlua::Error::UserDataTypeMismatch),
    }
}

pub fn open_iced(lua: &mlua::Lua) -> mlua::Result<()> {
    let iced = lua.create_table()?;
    let globals = lua.globals();
    // Lengths
    iced.set(
        "Fill",
        lua.create_function(|_lua, ()| -> mlua::Result<LuaLength> {
            Ok(LuaLength(iced::Length::Fill))
        })?,
    )?;
    iced.set(
        "FillPortion",
        lua.create_function(|_lua, val: u16| -> mlua::Result<LuaLength> {
            Ok(LuaLength(iced::Length::FillPortion(val)))
        })?,
    )?;
    iced.set(
        "Shrink",
        lua.create_function(|_lua, ()| -> mlua::Result<LuaLength> {
            Ok(LuaLength(iced::Length::Shrink))
        })?,
    )?;
    iced.set(
        "Fixed",
        lua.create_function(|_lua, val: f32| -> mlua::Result<LuaLength> {
            Ok(LuaLength(iced::Length::Fixed(val)))
        })?,
    )?;
    // Padding
    iced.set(
        "padding",
        lua.create_function(|_lua, val: f32| -> mlua::Result<LuaPadding> {
            Ok(LuaPadding(iced::Padding::new(val)))
        })?,
    )?;
    // Widgets
    iced.set(
        "container",
        lua.create_function(|_lua, val: mlua::Value| -> mlua::Result<LuaContainer> {
            Ok(LuaContainer(container(value_to_element(val)?).into()))
        })?,
    )?;
    iced.set(
        "column",
        lua.create_function(|_lua, val: mlua::Value| -> mlua::Result<LuaColumn> {
            match val {
                mlua::Value::Table(t) => {
                    let list: Vec<iced_core::Element<'static, Message, Theme, Renderer>> = t
                        .sequence_values::<mlua::Value>()
                        .map(|v| value_to_element(v.unwrap()).unwrap())
                        .collect();
                    Ok(LuaColumn(iced_widget::Column::from_vec(list)))
                }
                mlua::Value::Nil => Ok(LuaColumn(iced_widget::Column::new())),
                _ => Err(mlua::Error::BadArgument {
                    to: Some(String::from("iced.column")),
                    pos: 1,
                    name: Some(String::from("tbl")),
                    cause: std::sync::Arc::new(mlua::Error::UserDataTypeMismatch),
                }),
            }
        })?,
    )?;
    iced.set(
        "button",
        lua.create_function(|_lua, val: mlua::Value| -> mlua::Result<LuaButton> {
            Ok(LuaButton(button(value_to_element(val)?).into()))
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
