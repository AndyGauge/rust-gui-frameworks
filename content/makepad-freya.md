Makepad and Freya are both GPU-era toolkits that refuse to share much with the rest of the stack, in very different ways. **Makepad** builds everything itself: its own windowing, its own rendering, its own layout and its own shader language. Its lockfile contains no winit, no wgpu, no Taffy and no AccessKit. You write the UI in a live-reloading DSL, `live_design!`, and handle events in Rust.

::code makepad-app/src/bin/one.rs

The DSL is also where Makepad's distinctive feature lives: shaders are written inline. The `pixel` function below is a GPU fragment shader, in Makepad's own shading language, that styles a plain `View` with a gradient.

::code makepad-app/src/bin/two.rs

**Freya** went the other way and is easy to misdescribe. Earlier versions were "Dioxus plus Skia". Since 0.4 (16 July 2026) it has its own reactive and component model, no `dioxus-core` in its lockfile, and its own layout engine (`torin`). It does still render with Skia, through a fork called `freya-skia-safe`, with winit for windows and AccessKit for screen readers. UI is built by chaining plain Rust methods, with no macro.

::code freya-app/src/bin/one.rs
