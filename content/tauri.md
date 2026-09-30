Tauri does not draw your UI. It pairs a Rust backend with the **operating system's webview**: WKWebView on macOS, WebView2 on Windows, WebKitGTK on Linux. No browser is bundled, so apps are small. The Rust side exposes *commands* the web frontend can call by name. This is the full backend for a tiny app.

::code tauri-app/src/lib.rs

Note that `run` itself returns nothing, because mobile entry points cannot return a `Result`. It calls a fallible `try_run`, prints the error chain once and exits non-zero. The lockfile shows how the layers divide: `tauri-runtime-wry` brings in **wry** (the webview wrapper) and **tao** (windows). There is no winit, because tao is Tauri's fork of it. The frontend can be anything that runs in a browser: plain JavaScript, Svelte (this book is built with it), or a Rust framework such as Leptos or Yew compiled to WebAssembly.

::inline javascript | label=frontend (any JS framework)
import { invoke } from '@tauri-apps/api/core';

// Calls the Rust `greet` command by name and awaits the result.
const message = await invoke('greet', { name: 'Ada' });
console.log(message); // "Hello, Ada! (from Rust)"
::end

The Rust snippet was compiled as a real Tauri 2 project, including its build script, config and capability file. The JavaScript shows the matching call and is not compiled. Measured by total downloads (about 33 million) Tauri is the most downloaded framework here, ahead of egui's 25 million, with 680 dependent crates. Its 1.0 and 2.0 releases each have a page of their own.
