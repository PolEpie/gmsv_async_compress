# How does this work?
It use [gmod-lzma-rs](https://github.com/WilliamVenner/gmod-lzma-rs) and make it available for Garry's Mod with threading

## Lua Function

### async_compress()
	async_compress("Data to compress", function(compressed)
		print(compressed)
	end)

### async_decompress()
	async_decompress("Data to decompress", function(decompressed)
		print(decompressed)
	end)

## How to compile this?
### Linux 32bit
For Linux 32 bit, you need to do this in 2 stages:
1. Create a Docker image - `docker build -t rust_32bit .`
2. Execute the compilation - `docker run -it -v $(pwd):/compile_area -w "/compile_area" rust_32bit`
3. You can find the result in `target/i686-unknown-linux-gnu/release/libgmsv_voice_optimization.so` which you should copy to `lua/bin/gmsv_voice_optimization_linux.dll`
4. Invoke the module via `require("voice_optimization")`