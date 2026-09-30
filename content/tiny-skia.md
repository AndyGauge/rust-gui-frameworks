tiny-skia is a pure-Rust CPU rasterizer that covers a subset of Skia's feature set: paths, strokes, gradients, patterns and clipping. There is no GPU, no window and no C++ dependency. You create a `Pixmap`, fill shapes into it and save or display the pixels.

::code tiny-skia-draw/src/bin/one.rs

Gradients, strokes and blends work the same way. Because it is deterministic and dependency-free it is also used as a reference renderer in tests.

::code tiny-skia-draw/src/bin/two.rs

It appears in fourteen of the lockfiles we generated, often where you would not expect. On Linux, **winit itself** pulls it in: `winit → sctk-adwaita → tiny-skia` draws client-side window decorations on Wayland. Above that, `resvg` renders SVG with it, iced keeps `iced_tiny_skia` as a software fallback, and Floem's README describes a tiny-skia CPU renderer for machines without a GPU. It is the quiet "no GPU required" layer of the ecosystem.
