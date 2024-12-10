pub struct NLua {
    pub lua: mlua::Lua,
}

fn open_naev(lua: &mlua::Lua) -> mlua::Result<()> {
    let globals = lua.globals();
    let naev_table = lua.create_table()?;
    naev_table.set(
        "quit",
        lua.create_function(|_lua, ()| -> mlua::Result<()> {
            crate::quit();
            Ok(())
        })?,
    )?;
    globals.set("naev", naev_table)?;

    Ok(())
}

impl NLua {
    pub fn new() -> NLua {
        let lua = mlua::Lua::new();

        open_naev( &lua ).unwrap();

        NLua { lua }
    }
}
