# Rust GPU Framework
A basic framework for general programming with the GPU in Rust. For simplicity, the framework is targeted at Windows, Linux and MacOS.

## Structure
The frameworks folder contains most of the framework for interacting with the GPU through [wgpu](https://wgpu.rs/). The ```frameworks::windowed``` library is pretty much a decluttered clone of the framework used for the [wgpu examples](https://github.com/gfx-rs/wgpu/tree/trunk/examples) with added type annotations, and is intended for use in graphical applications. 

The ```frameworks::windowless``` is also based off the wgpu examples, and is primarily for GPU computation.

## Requirements
### OS
This framework was limited to Windows, Linux and MacOS targets for simplicity. The config.toml file attempts to use the mold linker on Linux and the lld linker on Windows.

### Crates
This framework relies on the crates mentioned in Cargo.toml:
- [bytemuck]() v1.14 with the ```derive``` feature | For byte conversion
- [log]() v0.4 | For logging
- [pollster]() v0.3 | For blocking and running async functions
- [rustc-hash]() v1.1 | For efficient hashing of shader related things
- [simple_logger]() v4.3 | For logging
- [web-time]() v1.1 | For frame-time diagnostics
- [winit]() v0.29 | For windowed applications
- [wgpu]() v0.19 | For communication with GPU

## Graphics Platform
The supported graphics platforms are the same as those for wgpu.

| API    | Windows             | Linux                  | MacOS                                                          |
|--------|---------------------|------------------------|----------------------------------------------------------------|
| Vulkan | &#x2705;            | &#x2705;               | &#x1F5F8; ([MoltenVK](https://vulkan.lunarg.com/sdk/home#mac)) |
| Metal  |                     |                        | &#x2705;                                                      |
| DX12   | &#x2705;            |                        |                                                                |
| OpenGL | &#x1F5F8; (GL 3.3+) | &#x1F5F8; (GL ES 3.0+) | &#x1F5F8; ([ANGLE](#angle))                                     |
