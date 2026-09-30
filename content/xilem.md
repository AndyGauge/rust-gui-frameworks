Xilem is the Linebender team's successor to Druid, and it splits into three layers. **Xilem** proper is a reactive view layer: a pure function from your state to a lightweight description of the UI, which it diffs on each change. **Masonry** is the retained widget toolkit underneath. **Vello** and **Parley** draw and lay out text. A counter looks like this.

::code xilem-counter/src/bin/one.rs

State can be split into reusable components that each see only one field, using `lens`, a descendant of Druid's `Lens`. The component below knows nothing about likes or dislikes; it only edits a `u32`. (The `+ use<>` in the signatures is Rust's precise-capturing syntax, which the crate's own examples use.)

::code xilem-counter/src/bin/two.rs

Its crates.io dependencies are `masonry`, `masonry_winit`, `vello` and `winit`. In our lockfile that resolves Vello **0.6.0**, Parley **0.6.0**, AccessKit **0.21.1** and wgpu **26.0.1**, all older than each crate's latest release, because Xilem 0.4 was released against them. It is the clearest example of a framework that sits on top of shared layers and lags them by several versions. Xilem is at 0.4.0, pre-1.0, with 6 dependent crates.
