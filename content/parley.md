Parley is the Linebender project's rich-text layout engine, the sibling of cosmic-text. You build a layout from a string, attach styles to ranges, break it into lines at a width, and get back runs of glyphs ready for a renderer. The builder borrows a `FontContext` and a `LayoutContext`, which you keep around so they can cache work between frames.

::code parley-layout/src/bin/one.rs

Styling applies to byte ranges, keeping text and style separate. A finished layout is organised as lines, then runs of same-font glyphs. Those are precisely what Vello-family renderers (and any other) draw.

::code parley-layout/src/bin/two.rs

Parley depends on `fontique` (font selection), `harfrust` (shaping), `skrifa`, and `accesskit` (so text layouts can describe themselves to screen readers). It is the text engine for Xilem and for Blitz/Dioxus Native (both on 0.6.0 in our lockfiles), and Slint's lockfile resolves Parley 0.11.1. Linebender's Q1 2026 report also lists Bevy among its adopters.
