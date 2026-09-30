Taffy is a layout engine: give it a tree of boxes with CSS-style properties (flexbox, grid, block) and it computes every box's size and position. It doesn't draw and doesn't know about windows. It grew out of the earlier `stretch` crate (2018) and is now the most widely shared layout layer in the book. Here is a 800×600 window with a fixed 200-pixel sidebar and content that fills the rest.

::code taffy-flex/src/bin/one.rs

Text is the tricky part, because a label's width depends on its font. Taffy handles that with a *measure function*: leaf nodes carry a context, and Taffy calls back to ask how big the content wants to be. A real toolkit calls its text engine there (cosmic-text, Parley). This one uses a fake 8 pixels per character.

::code taffy-flex/src/bin/two.rs

gpui, Floem and Blitz (under Dioxus Native) depend on it directly, and Slint's lockfile includes it too. But shared by name does not mean shared by version. `cargo tree -i taffy` on two frameworks, against the latest release of 0.14:

::code extra/taffy-gpui.txt | label=gpui-hello: cargo tree -i taffy
::code extra/taffy-floem.txt | label=floem-counter: cargo tree -i taffy

Dioxus Native resolves 0.9.2 and Slint 0.10.1. That is four different Taffy versions across four frameworks, each the latest release of its framework, and none of them 0.14. Not every framework uses it: Freya has its own engine (`torin`), and Makepad has no Taffy in its lockfile at all.
