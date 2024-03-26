cargo build --release --target i686-pc-windows-msvc
if (Test-Path target/i686-pc-windows-msvc/release/gmsv_async_compress.dll)
{
    mv target/i686-pc-windows-msvc/release/gmsv_async_compress.dll gmsv_async_compress_win32.dll
}