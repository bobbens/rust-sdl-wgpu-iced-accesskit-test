pub struct NLua {
    pub lua: mlua::Lua,
}

impl NLua {
    pub fn new() -> NLua {
        let lua = mlua::Lua::new();
        NLua { lua }
    }
}
