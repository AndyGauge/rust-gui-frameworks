egui is *immediate mode*: each frame you call functions that both describe and handle the UI. There is no widget tree to keep in sync with your state. `ui.button("Click me")` draws the button and, in the same call, tells you whether it was clicked. For tools, debuggers and game overlays that is hard to beat.

::code egui-app/src/bin/one.rs

What makes egui portable is that it knows nothing about windows or GPUs. It turns your UI into shapes, and a separate integration renders them. The snippet below runs one frame with no window at all and hands back triangle meshes plus texture updates.

::code egui-app/src/bin/two.rs

The integrations are separate crates, and the lockfile shows them: `egui-winit` for input, `egui-wgpu` for drawing (with `glow` as an OpenGL option) and `accesskit_winit` for screen readers. egui itself depends on AccessKit directly. That layering is why egui runs in a browser, in Bevy and inside game engines. It is the most downloaded toolkit that draws its own pixels (25 million total downloads, 1,236 dependents), and it is still 0.36, not 1.0.
