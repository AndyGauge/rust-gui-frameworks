wgpu is a Rust implementation of the WebGPU API that runs on Metal, Vulkan, Direct3D 12, OpenGL and, in a browser, the real WebGPU. Application code is the same everywhere. The first step is always the same: create an instance, ask for an *adapter* (a physical GPU), then open a *device* and a *queue*. The output below is from the Mac that built this book.

::code wgpu-device/src/bin/one.rs

The adapter reports which backend it used: here an Intel GPU through Metal. Note the constructor: in wgpu 30 an `InstanceDescriptor` has no `Default`; you pick `new_without_display_handle()` or `new_with_display_handle()`, so that the OpenGL backend can be told whether a display exists. This is the sort of change that shows up in every major version.

::code wgpu-device/src/bin/two.rs

`force_fallback_adapter` asks for a software implementation, the route a "no GPU available" mode would take. On this Mac none exists and wgpu says exactly why. That is the reason several frameworks keep a CPU renderer (Floem's README describes one built on tiny-skia, and Slint has a software renderer) and a reason software paths keep appearing in this ecosystem. wgpu is now the common GPU layer under egui, Slint, Vello and Floem, and versions 29 and 30 shipped in 2026.
