Floem came out of **Lapce**, a Rust code editor, and aims to be fast and ergonomic at once. Its model is *fine-grained reactivity*, the same idea as Leptos: state lives in signals, and only the closures that read a signal re-run when it changes. This is the counter from its README.

::code floem-counter/src/bin/one.rs

The view tree is built exactly once. After that, label text and styles are closures that re-evaluate on their own, so a state change never rebuilds the tree. That is the performance argument, and it is why the README credits Xilem, Leptos and `rui` as inspirations.

::code floem-counter/src/bin/two.rs

The README says rendering uses wgpu, with a CPU fallback built on tiny-skia "in case a GPU is unavailable". The lockfile agrees and adds specifics: Floem ships its own winit fork (`floem-winit`), uses Taffy **0.4.4** for layout and `cosmic-text` **0.12.1** for text, and resolves wgpu **22.1.0**, which is eight major versions behind wgpu 30. It has no AccessKit support, consistent with the 2026 ecosystem report. It is at 0.2.0 with 11 dependents.
