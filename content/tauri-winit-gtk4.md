Tauri windows come from **tao**, a fork of winit. The fork exists because on Linux a webview (WebKitGTK) is a GTK widget and needs a GTK event loop, which winit does not provide. The cost is that tao stays frozen on an older winit API while winit itself redesigned its interface for 0.31. Per the July–August 2026 ecosystem report, a winit-based GTK4 backend for Tauri merged on 16 July 2026 against the winit 0.31 beta.

::code tauri-app/src/lib.rs

Whichever event loop creates them, Tauri's windows expose the same handle traits as everything else. The `setup` hook above asks the main window for its window handle, the very same `HasWindowHandle` interface described on the raw-window-handle page. It is why a renderer written for winit windows could, in principle, draw into a Tauri one.

::code tauri-app/Cargo.toml

The same report names Linux shell integration as the ecosystem's hardest unsolved area: muda menu bars require GTK windows, tray-icon needs a parallel GTK loop, and global-hotkey only supports X11. On macOS and Windows none of this applies, which is why the backends diverge by platform. Treat the merge as a sign of direction and not as a finished fix. It is built on a beta.
