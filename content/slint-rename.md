Slint started life as **SixtyFPS** (first crates.io publish October 2020) and was renamed in early 2022; the `slint` crate first appears in February 2022. For users the migration was mostly a rename. This diff is illustrative and was not compiled.

::inline diff | label=migration (illustrative, not compiled)
-sixtyfps = "0.2"
+slint = "1"

-sixtyfps::sixtyfps! { ... }
+slint::slint! { ... }

-sixtyfps_build::compile("ui/app.60")
+slint_build::compile("ui/app.slint")
::end

Slint's idea is a separate declarative language, `.slint`, compiled to Rust at build time. Properties and callbacks are declared in the UI file, and your Rust code supplies behaviour by attaching handlers. This snippet, using the `slint!` macro, compiles against Slint 1.18.

::code slint-app/src/bin/two.rs

The lockfile shows a toolkit that keeps its options open: a winit backend, **three** renderers (`femtovg` for OpenGL and a `software` renderer are the defaults; Skia is also available), AccessKit for screen readers, and in our resolution it already depends on wgpu, Parley and Taffy. It is offered under the GPLv3, a royalty-free license, or a commercial license.
