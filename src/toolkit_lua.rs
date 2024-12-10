use crate::toolkit;
use crate::toolkit::Message;
use iced_core::Theme;
use iced_wgpu::Renderer;

pub fn open_iced(lua: &mlua::Lua) -> mlua::Result<()> {
    let globals = lua.globals();
    let iced = iced_lua::exports_table(lua)?;
    // Run function
    iced.set(
        "_run",
        lua.create_function(|_lua, (update, view): (mlua::Function, mlua::Function)| {
            toolkit::MESSAGE_QUEUE
                .lock()
                .unwrap()
                .push(Message::OpenLua(ToolkitWindowLua::new(update, view)?));
            Ok(())
        })?,
    )?;
    globals.set("iced", iced)?;
    // mlua doesn't support this directly, so we hack around it
    lua.load(
"iced.run = function (...)
    iced._run(...)
    coroutine.yield()
end ",
    )
    .exec()?;
    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToolkitWindowLua {
    update: mlua::Function,
    view: mlua::Function,
}

impl ToolkitWindowLua {
    pub fn new(update: mlua::Function, view: mlua::Function) -> mlua::Result<ToolkitWindowLua> {
        Ok(ToolkitWindowLua { update, view })
    }
}

impl toolkit::Window for ToolkitWindowLua {
    fn update(&mut self, message: Message) -> Message {
        match message {
            Message::Lua(m) => {
                let finish = self.update.call::<bool>(m.0).unwrap_or_else(|err| {
                    panic!("{}", err);
                });
                if finish {
                    return Message::CloseWindow;
                }
            }
            _ => unreachable!(),
        }
        Message::None
    }

    fn view(&self) -> iced_core::Element<Message, Theme, Renderer> {
        iced_lua::value_to_element(self.view.call::<mlua::Value>(()).unwrap_or_else(|err| {
            panic!("{}", err);
        }))
        .unwrap_or_else(|err| {
            panic!("{}", err);
        })
        .map(|m| Message::Lua(m))
    }
}
