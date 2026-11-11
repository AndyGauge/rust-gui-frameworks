zgui is one of the youngest frameworks in this book. Its repository opened on 2 August 2026 and `zgui` 0.1.0 was published on 15 October. It combines two ideas the book has already met separately: **Leptos's** reactivity and a **browser engine's** rendering pipeline. Components run once. Signals then update only the nodes that read them, and every stage after that keeps whatever work it can reuse. That covers style, layout, text, paint and pixels.

::code zgui-counter/src/bin/one.rs

This is not a lookalike of Leptos. zgui depends on Leptos's own `reactive_graph` crate, pinned to exactly 0.2.14, so signals, memos and effects behave as they do in Leptos. The stylesheet is also real CSS: selectors, the cascade, custom properties, flexbox, grid and transitions, resolved by **Stylo**, the style engine from Servo and Firefox. There is no HTML or DOM, though. zgui has a small native element vocabulary (`column`, `text`, `control`), and CSS selects those elements.

::code zgui-counter/src/bin/two.rs

The second crate, `zgui-ui`, ports most of shadcn/ui and themes it with design tokens. Changing the theme at runtime is a signal write. This matters because most toolkits in this book spend years before they have a usable widget set. zgui shipped with one, and the README's screenshots show a Kubernetes client, a music player and a code editor built on it.

The lockfile puts zgui very close to **Blitz** in this book's stack: Stylo for style, Parley **0.11.1** for text, Vello **0.9.0** for painting, AccessKit **0.24.1**, winit **0.30.13** and wgpu **29.0.4**. The difference is layout. zgui does not use Taffy. It uses **cephal**, an incremental layout engine by the same author, published on 10 October 2026, which patches only the dirty parts of the box tree. zgui also ships its own Wayland backend next to the winit one. The README warns that it is experimental and will break between 0.x releases. On 11 November 2026 it had 309 GitHub stars and fewer than 200 downloads.
