WebGPU is the browser API that wgpu is modelled on, and "shipped in every major browser" is true only at the level of the headline. Chrome and Edge shipped it in version 113 (2023) on Mac, Windows and ChromeOS. Safari 26 turned it on by default across macOS, iOS, iPadOS and visionOS. Firefox 141 shipped it on Windows. web.dev declared it supported in all major browsers in late 2025. For Rust, the browser target stopped being a pure bet, but it is still a bet that depends on who your users are. The same adapter-and-device code from the desktop page compiles for the web. This is the entire program, compiled for `wasm32-unknown-unknown`.

::code wgpu-web/src/lib.rs

Look at the error path: `request_adapter` can fail, and in a browser that is the normal outcome for many users. Per the [gpuweb implementation-status wiki](https://github.com/gpuweb/gpuweb/wiki/Implementation-Status) (last updated 13 August 2026), the gaps are large. **Firefox** ships on Windows, and on Apple Silicon Macs (145 on macOS 26 or later, 147 on all versions), but other Macs are Nightly only, Linux is Nightly only (Mozilla expects to ship in 2026), and Android is behind a flag. **Chromium** on Linux is limited to certain GPUs: Intel Gen12 and newer from 144, NVIDIA on Wayland from 147, and everything else needs launch flags. Windows on ARM64 is behind a flag, and Android support depends on the GPU vendor and Android version. **Servo** is still in progress. The Intel Mac that built this book's snippets falls under Firefox's "other Macs".

There is a second reason the headline overstates it: browsers do not update evenly. A feature that shipped in a release can take many months to reach most of your users, and managed corporate machines, older operating systems and phones that no longer get updates may never get it. So treat the support tables as an upper bound, and plan for a fallback. You choose the browser backend with a Cargo feature. `webgpu` targets the real API, while `webgl` remains as the fallback for browsers without it. Here only `webgpu` is enabled, and the crate builds as a `cdylib`, the WebAssembly module.

::code wgpu-web/Cargo.toml

On the JavaScript side, feature detection is one line, and is how a page decides which path to take. A missing `navigator.gpu` is not the only failure: `requestAdapter()` can also resolve to `null` on a browser that exposes the API but has no supported GPU.

::inline javascript | label=feature detection (illustrative, not compiled)
if (!navigator.gpu) {
  // No WebGPU at all: fall back to WebGL2 or a CPU renderer.
}
const adapter = await navigator.gpu.requestAdapter();
if (adapter === null) {
  // API present, but no usable GPU on this browser/OS/architecture.
}
::end

There is one irony worth noting for this book: the Firefox and Servo implementations are themselves built on the `wgpu` Rust project, per the same wiki, while Chromium's is the C++ Dawn project. egui (wgpu 30.0.1), Slint (30.0.1) and Vello (29.0.4) all resolve a recent wgpu, so the same browser backend is available to them. A standard shipping in every major browser is a real milestone. It is not yet a guarantee that a given user's browser has it switched on.
