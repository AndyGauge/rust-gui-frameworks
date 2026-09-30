Tauri 2.0 (2 October 2024) added **mobile** (iOS and Android) and replaced the global allowlist with a *permissions and capabilities* model. The Rust entry point changes shape: the app is a library with a `run` function, and a `mobile_entry_point` attribute lets the same code start as an Android activity or an iOS app. Desktop just calls `run` from `main`.

::code tauri-app/src/lib.rs

Permissions are declared per window in a capability file. Each capability names which windows it applies to and which plugin permissions they hold. The build script validates it at compile time, so an unknown permission is a build error. This is the exact file used in the project that compiled the Rust above.

::code tauri-app/capabilities/default.json

Tauri has 680 dependent crates and the largest download count of any framework in the book. 2.0 is a declared-stable release, so it carries the green badge, but the line is moving again: a 3.0 alpha is already on crates.io as of September 2026. Going from 1.0 to 2.0 took about two and a half years.
