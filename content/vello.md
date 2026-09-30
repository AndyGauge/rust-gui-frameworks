Vello is a 2D vector renderer that does the heavy lifting on the GPU using compute shaders rather than the traditional rasterization pipeline. You do not draw with it directly. You record drawing commands into a `Scene`, a list of fills and strokes with transforms, and hand that to a renderer. The shapes and colours come from two sibling crates, `kurbo` (geometry) and `peniko` (brushes).

::code vello-scene/src/bin/one.rs

Turning a scene into pixels needs a wgpu device and a texture to draw into. Notice where `wgpu` comes from: Vello re-exports it as `vello::wgpu`, because the wgpu version you use must match Vello's exactly. That requirement is the version-skew problem in miniature.

::code vello-scene/src/bin/two.rs

Vello's dependencies are `peniko`, `skrifa` (font outlines) and `wgpu`. Its users pin older releases: Xilem 0.4 and Dioxus Native both resolved **Vello 0.6.0**, while the crate itself is at 0.10.0. The project has also split into a family: Vello Hybrid (GPU compositing with CPU geometry) and Vello CPU (no GPU), according to Linebender's Q1 2026 report, with Masonry moving to an `imaging` abstraction so it can use either.
