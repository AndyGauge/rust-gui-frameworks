Dioxus (2021) brings a React-style component model to Rust: components are functions returning `rsx!` markup, and state lives in signals. One component tree can target the web (as WebAssembly), the desktop, mobile, and, since 2025, a renderer that does not use a browser at all. First, the smallest desktop app.

::code dioxus-app/src/bin/one.rs

Props are function arguments, loops and conditionals are plain Rust, and `count += 1` on a signal schedules a re-render of exactly the parts that read it. Note that `dioxus::launch` is the only line that names a platform; the components around it are portable.

::code dioxus-app/src/bin/two.rs

"Desktop" in Dioxus has meant a **webview**: our lockfile for the `desktop` feature contains `dioxus-desktop`, `wry`, `tao` and `muda`, the same window and webview layers Tauri uses. Your Rust draws into the operating system's browser engine. The alternative renderer, Dioxus Native, is covered on its own page. The stable line today is 0.7 (0.8 is in alpha), and the crate has not reached 1.0.
