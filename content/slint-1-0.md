Slint 1.0 arrived on 3 April 2023, about two and a half years after the first `sixtyfps` publish. Like Tauri's 1.0 it is a public-API promise: the `.slint` language and the language bindings will not break within 1.x. Here is the UI, as a separate file in the `.slint` language.

::code slint-file/ui/app.slint

A build script compiles that file into Rust at build time, and `include_modules!` makes the generated `App` type available. You then attach behaviour with generated setters, getters and callbacks. The SixtyFPS-to-Slint page shows the alternative, which embeds the same language in a macro.

::code slint-file/build.rs
::code slint-file/src/main.rs

The promise is holding: Slint is at **1.18.1** today, eighteen minor releases after 1.0 and all of them 1.x. Both the file-based project and the macro version compile against 1.18. The cost of the stability is a language and compiler separate from Rust, and a licensing model (GPLv3, royalty-free, or commercial) that you need to read.
