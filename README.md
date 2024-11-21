# mach-dxcompiler-rs

This library allows for statically linking *prebuilt binaries* from [`mach-dxcompiler-rs`](https://github.com/hexops/mach-dxcompiler) into a Rust project. The `mach-dxcompiler` repository is a fork of Microsoft's [`DirectXShaderCompiler`](https://github.com/microsoft/DirectXShaderCompiler/tree/main) that replaces the CMake build system with Zig. This allows for building the project statically and linking it into existing applications - but the core logic comes from the original DXC library.

### Usage

First, add this crate to the project: `cargo add mach-dxcompiler-rs`

Next, use the `DxcCreateInstance` function to create DXC COM objects. Once created, these objects are usable with the normal COM windows API.

```rust
use windows::Win32::Graphics::Direct3D::Dxc;

let mut obj = std::ptr::null_mut();
mach_dxcompiler_rs::DxcCreateInstance(&Dxc::CLSID_DxcCompiler, &Dxc::CLSID_DxcUtils, &mut object);
// ... use the shader compiler ...
```