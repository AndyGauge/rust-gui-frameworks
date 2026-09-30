Yew (December 2017) is the oldest surviving Rust UI framework in this book, and it targets the browser, not a native window. Components are plain functions, markup is written with the `html!` macro, and state uses hooks like `use_state`. The `csr` (client-side rendering) feature is what turns a component tree into DOM nodes.

::code yew-counter/src/bin/one.rs

Components compose like HTML elements and their props are checked at compile time, so a typo in a prop name is a build error rather than a runtime surprise. Both snippets here were compiled for the `wasm32-unknown-unknown` target, the one a browser runs.

::code yew-counter/src/bin/two.rs

Yew sits on `wasm-bindgen`, `web-sys` and `js-sys` and nothing else from the graphics stack: no winit, no wgpu, no layout engine. The browser does all of that. It is still maintained at 0.23 with 318 crates depending on it, but it is pre-1.0, and most newcomers to Rust on the web now reach for Leptos or Dioxus.
