Blinc's repository dates back to 2022, but its first crates.io release, `blinc_app` 0.1, came on 14 January 2026. There have been nine releases in all, the latest 0.6.0 on 4 November. It is a GPU-first toolkit with a builder API that reads like Tailwind class names.

::code blinc-app/src/bin/one.rs

The same `ui` function runs on desktop and in the browser through WebGPU, and the README lists Android and iOS as stable and HarmonyOS as in progress. Blinc's selling point is visual polish. It renders shapes as signed distance fields, ships glassmorphism and spring physics, and lets you add custom shaders.

::code blinc-app/src/bin/two.rs

Reactivity uses signals keyed by call site (`ctx.use_state`), and a `stateful` block re-renders only when the signals listed in its `deps` change. Blinc also has first-class state machines (`stateful::<S>()` with hover, press and drag states), which its README lists as a headline feature. Like zgui and gpui-component, it ships a shadcn-style component library, `cn`, with more than 40 components. It also ships a `Skills.md` written specifically for coding agents.

The lockfile shows Blinc keeping the bottom layers and building the rest itself. It uses winit **0.30.13** and Taffy **0.14.0**, and pins wgpu with `=26.0.1`, an exact pin four major versions behind wgpu 30. Text is Blinc's own crate, built on `rustybuzz` and `ttf-parser` rather than Parley or cosmic-text. There is **no AccessKit** in the lockfile, so on 11 November 2026 Blinc had no screen-reader support. It had 570 GitHub stars and about 1,000 downloads.
