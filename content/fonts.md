Before a UI can shape or lay out text it needs glyphs: the outlines of individual letters, turned into pixel coverage. Rust's first popular answer was **rusttype** (2016), later deprecated in favour of **ab_glyph**, which does the same job with a smaller, faster API. Give it font bytes and a character, and it hands back an outline and, when drawn, a coverage value per pixel. There is no shaping and no layout. (Adjust the font path for your system.)

::code glyph-fonts/src/bin/one.rs

The older crate did the same work through a layout iterator that positioned glyphs one after another, which is the shape of the API that most 2016-era Rust GUIs were written against.

::code glyph-fonts/src/bin/two.rs

Both crates have been overtaken by the **fontations** family from the Google Fonts team: `read-fonts` and `skrifa` parse font files, and `swash` (which depends on skrifa) scales, rasterizes and shapes. In our lockfiles skrifa is everywhere: egui, iced, Slint, Xilem, Dioxus Native, Floem and gpui all resolve it, and Vello, Parley, cosmic-text and swash depend on it directly. The layers above it, shaping and layout, are the next two pages.
