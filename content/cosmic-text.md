cosmic-text (October 2022) was written by System76 for the COSMIC desktop. It takes a string, finds fonts on the system, *shapes* the text (choosing glyphs and positions, including right-to-left scripts and emoji) and *lays it out* into lines. A `Buffer` holds the text and its layout. This example mixes English, Hebrew and an emoji.

::code cosmic-shape/src/bin/one.rs

Spans can carry their own attributes such as family and weight, and the buffer wraps them to the width you give it. Careful with examples online: this API has changed between releases, and older code passes the font system to `set_size` and `set_text`. These snippets were compiled against 0.19.

::code cosmic-shape/src/bin/two.rs

Its dependencies are `fontdb` (finding system fonts), `skrifa` and `swash`, and `harfrust` for shaping, the Rust port of HarfBuzz that cosmic-text, Parley and egui all adopted within about a year. cosmic-text is one of four maintained text-layout stacks in the ecosystem. In our lockfiles it serves iced (0.15.0), gpui (0.14.2) and Floem (0.12.1), each on a different release, none of which is the current 0.19.
