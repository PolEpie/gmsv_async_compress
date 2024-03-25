use gmod::lua_string;

pub fn print(lua: &gmod::lua::State, msg: &String) {
	unsafe {
        lua.get_global(lua_string!("print"));
        lua.push_string(msg);
        lua.call(1, 0);
    }
}

// Create a public queue of messages, static singleton