raw-window-handle is one tiny crate with one job: describe the native object behind a window in a form any library can accept. On macOS that is an `NSView`, on Windows an `HWND`, on Wayland a `wl_surface`, and in a browser a canvas. It has no dependencies of its own to speak of, and yet 789 crates depend on it.

::code rwh-handle/src/bin/one.rs

The real value is in the traits. A renderer does not ask for a winit `Window`; it asks for something that implements `HasWindowHandle` and `HasDisplayHandle`. That is the entire contract between a window library and a graphics library.

::code rwh-handle/src/bin/two.rs

It is why you can mix and match: winit's window works with wgpu, glutin, and every toolkit above them, and Tauri's and tao's windows expose the same traits. In our lockfile survey it appears in every native framework we built except Makepad and Druid, which have their own platform layers. One wrinkle: the Dioxus desktop and Floem lockfiles resolved **both** 0.5.2 and 0.6.2, because some of their dependencies have not moved to the newer trait versions yet.
