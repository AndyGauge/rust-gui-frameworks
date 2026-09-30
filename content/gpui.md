gpui is the UI framework behind the Zed editor. Views are structs that implement `Render`. Their `render` method returns a tree of `div()`s styled with a Tailwind-like vocabulary: `.flex()`, `.gap_2()`, `.bg(...)`. Mutating state is explicit, and you call `cx.notify()` to say "this view changed, render it again."

::code gpui-hello/src/bin/one.rs

The styling methods are not CSS. Each one sets a Taffy style, and Taffy computes the layout before gpui paints. gpui depends directly on `taffy` (0.9.0), `cosmic-text` (0.14.2), `lyon` (tessellation) and `resvg`, and it has no winit: it has its own platform layers (Metal on macOS, plus Windows, X11 and Wayland backends).

::code gpui-hello/src/bin/two.rs

A caution about versions. The gpui on crates.io (0.2.x, published October 2025) is behind Zed's own repository: its lockfile still resolves the Blade graphics library, while Zed moved Linux rendering to wgpu in February 2026. On macOS it also compiles Metal shaders at build time, which needs Xcode's shader toolchain; the snippet above uses the `runtime_shaders` feature to defer that. Both examples compile against gpui 0.2.2 from crates.io. The startup closure returns nothing, so a failed `open_window` is reported and the app quits rather than panicking.
