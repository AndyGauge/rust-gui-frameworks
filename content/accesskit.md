Screen readers and other assistive tools need a description of your UI: what the buttons are, what they're labelled, which one has focus. For a toolkit that draws its own pixels, nothing provides that for free. AccessKit is a shared, toolkit-independent *tree of nodes* that a framework fills in, which platform adapters then expose to VoiceOver, Narrator, or AT-SPI on Linux.

::code accesskit-tree/src/bin/one.rs

Nodes declare their role, label and state. Declaring supported *actions* is how a screen reader learns it may click or focus something on the user's behalf. A note from the compiler: in 0.25 the old `Tree` type has been deprecated in favour of `TreeInfo`, another reminder to pin versions.

::code accesskit-tree/src/bin/two.rs

Adoption is broad but uneven in versions. The lockfiles resolve AccessKit **0.24.1** for egui, Freya and Slint, **0.21.1** for Xilem and **0.17.1** for Dioxus Native, against 0.25.1 today. Iced, Floem and Makepad have no AccessKit in their lockfiles at all, and the published gpui 0.2.2 doesn't either; the July 2026 ecosystem report says gpui gained it in May 2026, the same month AccessKit's iOS support arrived. The same report notes AccessKit has had a single active maintainer since March 2026.
