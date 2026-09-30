winit creates windows and delivers input: keyboard, mouse, touch, resize, close. It does not draw anything. That narrow job is why so much of this book is built on it. When we resolved the latest release of egui, iced, Slint, Xilem, Dioxus Native and Freya, **every one landed on winit 0.30.13**. Here is the smallest complete program.

::code winit-window/src/bin/one.rs

Version 0.30 replaced the old closure-based event loop with the `ApplicationHandler` trait. Windows are created in `resumed` rather than at startup because mobile and web platforms can suspend and resume an app, and the window may not exist yet when `main` runs. Every toolkit in this book, directly or indirectly, implements something like this handler.

::code winit-window/src/bin/two.rs

This is the seam where a window library meets a toolkit. `Resized` is where a renderer recreates its swapchain and a layout engine re-runs. `RedrawRequested` is where the toolkit paints. Some frameworks do not use winit at all: Tauri uses **tao**, its fork of winit; Floem ships a fork (`floem-winit`); and gpui, Makepad and Druid have their own platform layers. The crate's 0.31 beta has been open for months as of this writing, which makes winit the most load-bearing pre-1.0 crate in the stack.
