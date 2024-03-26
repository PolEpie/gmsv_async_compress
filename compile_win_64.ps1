cargo build --release --target x86_64-pc-windows-msvc
if (Test-Path target/x86_64-pc-windows-msvc/release/gmsv_async_compress.dll)
{
    mv target/x86_64-pc-windows-msvc/release/gmsv_async_compress.dll gmsv_async_compress_win64.dll
}