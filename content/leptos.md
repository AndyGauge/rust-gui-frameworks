Leptos (October 2022) takes a different route to the same React-shaped idea: **fine-grained reactivity**. A `signal` is a value that remembers who read it. When it changes, only those readers re-run, so there is no virtual DOM to diff. It targets the browser, both client-side (as here, the `csr` feature) and with server-side rendering plus hydration.

::code leptos-counter/src/bin/one.rs

Derived values such as the closure `doubled` and the `Memo` called `label` are just functions of signals. They recompute when, and only when, an input signal changes. The view macro then binds those values directly to DOM text nodes.

::code leptos-counter/src/bin/two.rs

Both snippets compile for `wasm32-unknown-unknown`. Leptos depends on `wasm-bindgen` and `web-sys` and nothing native, so it is the web-only framework in this book: it renders through the browser, and for a desktop app you would put it inside Tauri. The current release is 0.8.21 with 0.9 in beta, and the ecosystem around it (472 dependents) is the largest of the Rust web frameworks here.
