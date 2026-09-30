WebGPU is the browser API that wgpu is modelled on. Chrome and Edge shipped it in version 113 (2023), Firefox in version 141 and Safari in version 26, and web.dev declared it supported in all major browsers in late 2025, with Safari 26 the last to ship. For Rust, that matters because wgpu's browser target stops being a bet. The same adapter-and-device code from the desktop page compiles for the web. This is the entire program, compiled for `wasm32-unknown-unknown`.

::code wgpu-web/src/lib.rs

You choose the browser backend with a Cargo feature. `webgpu` targets the real API, while `webgl` remains as the fallback for browsers without it. Here only `webgpu` is enabled, and the crate builds as a `cdylib`, the WebAssembly module.

::code wgpu-web/Cargo.toml

On the JavaScript side, feature detection is one line, and is how a page decides which path to take:

::inline javascript | label=feature detection (illustrative, not compiled)
if (!navigator.gpu) {
  // Older browser: fall back to WebGL2 or a CPU renderer.
}
const adapter = await navigator.gpu.requestAdapter();
::end

egui (wgpu 30.0.1), Slint (30.0.1) and Vello (29.0.4) all resolve a recent wgpu, so the same browser backend is available to them. A standard shipping in every browser is a stability milestone of a different kind, because it is the one the whole stack can assume.
