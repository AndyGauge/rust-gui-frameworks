Rust 1.0 is a promise: code that compiles today will keep compiling on every later 1.x compiler. Everything in this book rests on it. But notice what the promise does **not** cover: your dependencies. Cargo reads version numbers below 1.0 differently, and for a `0.x` crate a *minor* bump is a breaking change. The next snippet asks Cargo's own version-matching library what `winit = "0.30"` accepts.

::code semver-rust1/src/bin/one.rs

Nothing newer than the 0.30 series matches, not even the 0.31 beta that has been in the works for months. That rule matters here because nearly every crate in this book is still `0.x`: winit 0.30, egui 0.36, iced 0.14, Taffy 0.14, AccessKit 0.25. wgpu is past 1.0 on paper, but it is at major version 30 because it ships a new breaking release every few months (29 in March 2026, 30 in July).

::code semver-rust1/src/bin/two.rs

So "stable" in this book means something specific: a framework that has declared 1.0 and promised not to break you. Of the frameworks checked, only Tauri, Slint and Makepad have. Look for the green badge on their pages. For everything else, pin your versions and read the changelog before you upgrade.
