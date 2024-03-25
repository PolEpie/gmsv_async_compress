#![feature(c_unwind)]
#![feature(extern_types)]

use std::env;

use gmod::{gmod13_open, gmod13_close, lua_string};
use util::print;
mod util;
mod func;
mod worker;
use std::cell::Cell;

thread_local! {
	static LUA: Cell<Option<gmod::lua::State>> = Cell::new(None);
}

#[gmod13_open]
unsafe fn gmod13_open(lua: gmod::lua::State) -> i32 {
    // Save to variable: "My {} module version {} has loaded!", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")
    let str = format!("Loading {}{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    print(&lua, &str);
	
	lua.push_function(func::async_compress);
    lua.set_global(lua_string!("async_compress"));

	lua.push_function(func::async_decompress);
    lua.set_global(lua_string!("async_decompress"));
	
    lua.pop();

	worker::init();

    print(&lua, &"Voice Module Loading completed".to_string());
	
	LUA.with(|cell| {
		cell.set(Some(lua));
	});
    0
}

#[gmod13_close]
unsafe fn gmod13_close(_lua: gmod::lua::State) -> i32 {
    return 0;
}
