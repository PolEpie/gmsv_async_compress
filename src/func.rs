
use gmod::{lua};

use crate::worker;

pub unsafe extern "C-unwind" fn async_compress(lua: gmod::lua::State) -> i32 {
	if lua.lua_type(1) != lua::LUA_TSTRING || lua.lua_type(2) != lua::LUA_TFUNCTION {
		println!("[ERROR] Invalid arguments passed to async_compress");
		return 0;
	}

	let data_to_compress = lua.get_binary_string(1).unwrap();
	let callback = lua.reference();
	lua.pop();
	
	/* let compressed = gmod_lzma::compress(data_to_compress, 9).unwrap();
	lua.push_binary_string(&compressed); */


	let arg = worker::Argument {
		callback,
		data: data_to_compress.to_vec(),
		operarion_is_compress: true,
	};
	
	worker::send(lua, arg);
	return 0;
}

pub unsafe extern "C-unwind" fn async_decompress(lua: gmod::lua::State) -> i32 {
	if lua.lua_type(1) != lua::LUA_TSTRING || lua.lua_type(2) != lua::LUA_TFUNCTION {
		println!("[ERROR] Invalid arguments passed to async_compress");
		return 0;
	}

	let data_to_compress = lua.get_binary_string(1).unwrap();
	let callback = lua.reference();
	lua.pop();
	
	/* let compressed = gmod_lzma::compress(data_to_compress, 9).unwrap();
	lua.push_binary_string(&compressed); */


	let arg = worker::Argument {
		callback,
		data: data_to_compress.to_vec(),
		operarion_is_compress: false,
	};
	
	worker::send(lua, arg);
	return 0;
}
