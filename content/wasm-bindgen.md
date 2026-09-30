wasm-bindgen generates the glue between Rust compiled to WebAssembly and JavaScript. `web-sys` builds on it to give Rust typed access to browser APIs: the DOM, canvas, fetch, WebGPU. It is the most depended-on crate in this book, with over 5,000 dependents, and nearly every web target here rests on it: Yew, Leptos, Sycamore, winit's web backend and wgpu's browser backend all list it.

::code wasm-bindgen-dom/src/bin/one.rs

Using a browser API takes only a few lines, and you must enable each `web-sys` type you use as a Cargo feature (here `Window`, `Document`, `Element`, `HtmlElement`). That keeps the compiled `.wasm` small, and it is why Rust web frameworks ask you to list features so carefully.

::code wasm-bindgen-dom/src/bin/two.rs

The boundary runs both ways. `extern "C"` blocks import JavaScript functions into Rust and `#[wasm_bindgen]` on a function exports it out. Build tools like Trunk or wasm-pack run the `wasm-bindgen` step for you. Both snippets were compiled for `wasm32-unknown-unknown`. The crate is still `0.2.x`, stable in practice and pre-1.0 on paper.
