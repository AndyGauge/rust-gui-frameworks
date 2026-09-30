Makepad's widget crate reached **1.0.0 on 13 May 2025**, more than two years after its first publish in October 2022. It is the smallest project in the book to have declared stability, with only 8 dependent crates, and unusual in doing so while still building every layer itself. Here is what depending on it looks like.

::code makepad-app/Cargo.toml

`makepad-widgets = "1.0"` means "any 1.x from 1.0 up". That is the whole effect of a 1.0: Cargo will now resolve any later 1.x release and, by the rules on the Rust 1.0 page, it is not supposed to break you. A `0.x` dependency cannot make that promise.

Stability here is a statement about the widget API, the `live_design!` DSL and the application structure. It is not a statement about adoption, since this crate has a small user base, and it tells you nothing about the platform backends, which Makepad maintains on its own rather than sharing with winit. A declared 1.0 is a real milestone, but read it as a promise from one team and not as evidence of ecosystem consensus.
