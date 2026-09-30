Tauri 1.0 shipped on 19 June 2022, after nine months of betas and four months of release candidates. Among the frameworks in this book it was the first to declare itself stable (FLTK's Rust bindings had reached a 1.x earlier), which is what "1.0" buys you: a promise that the public API will not break within the 1.x line. The 1.0 security model was an **allowlist**. Nothing the frontend could do was permitted by default, and you switched on each capability in `tauri.conf.json`.

::inline json | label=tauri.conf.json (Tauri 1.x; illustrative, not compiled)
{
  "tauri": {
    "allowlist": {
      "all": false,
      "fs": {
        "all": false,
        "readFile": true,
        "scope": ["$APPDATA/*"]
      },
      "dialog": { "open": true }
    }
  }
}
::end

That deny-by-default design was the point. A webview can run arbitrary JavaScript, including from a compromised dependency, so the Rust side decides what reaches the operating system. The allowlist was global to the app, which is the limitation that Tauri 2.0 replaced with per-window *capabilities*. The 1.x line is superseded but was widely deployed.
