iced follows the Elm architecture. Your program is a state type, a `Message` enum, an `update` function that applies messages, and a `view` function that builds widgets from state. Widgets never change state; they emit messages. That makes every state change explicit and easy to follow.

::code iced-counter/src/bin/one.rs

Things that happen outside the UI (timers, sockets, file watchers) arrive as a **subscription** that turns events into ordinary messages. In iced 0.14 the `time::every` helper needs an async runtime feature enabled, which is `tokio` in this project's manifest.

::code iced-counter/src/bin/two.rs

Underneath, the lockfile shows the stack iced assembles: `iced_winit` for windows, `iced_wgpu` for GPU drawing with `iced_tiny_skia` as a software fallback, and `cosmic-text` for text. System76's COSMIC desktop is built on a fork of iced. Version 0.14 arrived more than a year after 0.13, and iced has no AccessKit integration yet, unlike egui and Slint.
