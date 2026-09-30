// The book. Each section carries a `year` (and optional `month`) — reading order is chronological.
// Sequential page numbers (01, 02, …) are assigned after sort.
//
// DRAFT: dates are crates.io first-publish dates (crates.io API, Sep 2026) unless noted.
// That is NOT always project birth — verify repo/announcement dates before printing.
//
// topic = the layer: Windowing, Rendering, Text, Layout, Accessibility, Framework, Web.
// status = 2026 standing, -1 (abandoned) … 0 (important, few users / niche) … +1 (mainstream).
//          An editorial judgement informed by crates.io download counts — revisit before printing.
// stable = true marks a declared 1.0 / stability milestone (shown as a badge).
// kind  = 'primitive' (a layer other things build on) | 'framework'.

// One symbol per layer of the stack.
export const TOPIC_ICONS = {
  Windowing: '🪟',
  Rendering: '🎨',
  Text: '🔤',
  Layout: '📐',
  Accessibility: '♿',
  Framework: '🧩',
  Web: '🌐',
  Language: '🦀'
};

const CRATES = 'crates.io first publish.';

const raw = [
  // ---- Windowing ----
  {
    title: 'glutin: a window and a GL context',
    status: -0.4, statusNote: 'Superseded by winit plus helper crates',
    year: 2014, month: 11, topic: 'Windowing', kind: 'primitive',
    gesture: 'Before any toolkit, someone had to open a window from Rust.',
    body: 'glutin (2014) created windows and OpenGL contexts. It was the ancestor of the windowing layer that winit later split out.',
    citation: CRATES
  },
  {
    title: 'winit: the window everyone borrows',
    status: 0.9, statusNote: 'Near-universal, but 0.31 is stuck in beta',
    year: 2016, month: 3, topic: 'Windowing', kind: 'primitive',
    gesture: 'Almost every framework in this book stands on the same window library.',
    body: 'winit handles windows and input across platforms. egui, iced, Xilem, Slint and others use it, and Tauri forks it as **tao**. Its 0.31 beta has been open for months in 2026, which makes it a load-bearing risk.',
    citation: CRATES + ' See also the July–Aug 2026 ecosystem report.',
    link: 'https://goldstrikearch.github.io/rust-gui-desktop-ecosystem-state/'
  },
  {
    title: 'raw-window-handle: the handshake',
    status: 0.9, statusNote: 'Ubiquitous plumbing',
    year: 2019, month: 7, topic: 'Windowing', kind: 'primitive',
    gesture: 'A tiny crate that lets any window talk to any renderer.',
    body: 'raw-window-handle is a shared type for passing a native window to a graphics library. It decouples winit from wgpu and the rest.',
    citation: CRATES
  },

  // ---- Web ----
  {
    title: 'Yew: Rust in the browser',
    status: 0.0, statusNote: 'Maintained, overshadowed by Leptos',
    year: 2017, month: 12, topic: 'Web', kind: 'framework',
    gesture: 'The first popular Rust component framework targeted the DOM, not a native window.',
    body: 'Yew (Dec 2017) was an Elm/React-style framework. It came just before wasm-bindgen made compiling Rust to the web practical.',
    citation: CRATES
  },
  {
    title: 'wasm-bindgen: Rust meets the DOM',
    status: 1.0, statusNote: 'Foundational and universal',
    year: 2018, month: 3, topic: 'Web', kind: 'primitive',
    gesture: 'The bridge that makes every Rust web UI framework possible.',
    body: 'wasm-bindgen (and later web-sys) let Rust call browser APIs. Yew, Sycamore, Leptos and Dioxus-web all rely on it.',
    citation: CRATES
  },
  {
    title: 'Dioxus: React-shaped Rust',
    status: 0.6, statusNote: 'Growing and widely used',
    year: 2021, month: 1, topic: 'Framework', kind: 'framework',
    gesture: 'One component model, several renderers.',
    body: 'Dioxus (2021) targets web, desktop (via an OS webview), mobile and, more recently, a native renderer. Freya reuses its core with Skia.',
    citation: CRATES
  },
  {
    title: 'Leptos: fine-grained reactivity',
    status: 0.6, statusNote: 'Leading Rust web framework',
    year: 2022, month: 10, topic: 'Web', kind: 'framework',
    gesture: 'Signals, SSR and hydration, written in Rust.',
    body: 'Leptos (Oct 2022) takes a Solid-style signal approach for full-stack web apps. Sycamore (2021) shares that lineage.',
    citation: CRATES
  },

  // ---- Rendering ----
  {
    title: 'wgpu: the shared GPU layer',
    status: 1.0, statusNote: 'The default GPU layer',
    year: 2019, month: 1, topic: 'Rendering', kind: 'primitive',
    gesture: 'One GPU API for Vulkan, Metal, D3D and the browser.',
    body: 'wgpu implements WebGPU natively. It became the common renderer for Bevy, iced, egui, Vello and, in Feb 2026, Zed on Linux.',
    citation: CRATES
  },
  {
    title: 'iced: Elm for Rust',
    status: 0.5, statusNote: 'Real users via COSMIC; no AccessKit yet',
    year: 2019, month: 5, topic: 'Framework', kind: 'framework',
    gesture: 'A typed, message-driven toolkit that COSMIC is built on.',
    body: 'iced (May 2019) follows the Elm architecture. It is the basis of System76’s COSMIC desktop. Per the 2026 ecosystem report it has not yet integrated AccessKit.',
    citation: CRATES
  },
  {
    title: 'Tauri: the webview shell',
    status: 0.9, statusNote: 'The mainstream desktop choice',
    year: 2019, month: 11, topic: 'Framework', kind: 'framework',
    gesture: 'Ship a web UI with a Rust backend and the system webview.',
    body: 'Tauri (Nov 2019) uses **tao** (a winit fork) and **wry** (a webview wrapper, 2021). It skips custom rendering entirely.',
    citation: CRATES
  },
  {
    title: 'egui: immediate mode wins hearts',
    status: 1.0, statusNote: 'Most downloaded GUI toolkit',
    year: 2020, month: 5, topic: 'Framework', kind: 'framework',
    gesture: 'Immediate mode made Rust GUIs feel easy.',
    body: 'egui (May 2020, eframe Jan 2021) became the default way to add a quick UI to a Rust tool, game or debugger.',
    citation: CRATES
  },
  {
    title: 'tiny-skia: a software fallback',
    status: 0.8, statusNote: 'Widely depended on',
    year: 2020, month: 7, topic: 'Rendering', kind: 'primitive',
    gesture: 'Skia-style drawing with no GPU and no C++.',
    body: 'tiny-skia (Jul 2020) is a small CPU rasterizer. resvg (2017) uses it to render SVG.',
    citation: CRATES
  },
  {
    title: 'SixtyFPS becomes Slint',
    status: 0.6, statusNote: 'Commercially backed, embedded adoption',
    year: 2020, month: 10, topic: 'Framework', kind: 'framework',
    gesture: 'A declarative UI language for embedded and desktop.',
    body: 'The crate first appeared as sixtyfps in Oct 2020. The `slint` crate appeared in Feb 2022. It is dual licensed commercially and under the GPL.',
    citation: CRATES
  },
  {
    title: 'Vello: GPU compute vector rendering',
    status: 0.15, statusNote: 'Important but alpha and few direct users',
    year: 2024, month: 3, topic: 'Rendering', kind: 'primitive',
    gesture: 'Drawing paths with compute shaders instead of rasterization.',
    body: 'vello (first crates.io publish Mar 2024) grew out of earlier piet-gpu research. Xilem and other Linebender projects render with it. The CPU variant `vello_cpu` followed in May 2025.',
    citation: CRATES + ' Project origins predate the crate.'
  },

  // ---- Text ----
  {
    title: 'Fonts: rusttype to swash',
    status: -0.3, statusNote: 'rusttype is deprecated; successors carry on',
    year: 2016, month: 2, topic: 'Text', kind: 'primitive',
    gesture: 'Drawing text is its own ecosystem.',
    body: 'rusttype (2016) rasterized glyphs. ab_glyph (2020), fontdue (2019) and swash (2021) followed, then read-fonts/skrifa (2022–23) from the Google Fonts team.',
    citation: CRATES
  },
  {
    title: 'cosmic-text: shaping and layout',
    status: 0.7, statusNote: 'Widely adopted text stack',
    year: 2022, month: 10, topic: 'Text', kind: 'primitive',
    gesture: 'System76 builds a text engine for its own desktop.',
    body: 'cosmic-text (Oct 2022) does shaping, BiDi and layout. It is one of four maintained text-layout stacks in 2026.',
    citation: CRATES
  },
  {
    title: 'Parley: the Linebender text stack',
    status: 0.2, statusNote: 'Growing, tied to Linebender',
    year: 2024, month: 5, topic: 'Text', kind: 'primitive',
    gesture: 'Text layout for Xilem and friends.',
    body: 'parley and fontique (May 2024) provide rich text layout and font selection. HarfRust adoption by parley, cosmic-text and egui is the 2025–26 consolidation story.',
    citation: CRATES
  },

  // ---- Layout ----
  {
    title: 'stretch to Taffy: one layout engine',
    status: 0.9, statusNote: 'Shared across many frameworks',
    year: 2022, month: 6, topic: 'Layout', kind: 'primitive',
    gesture: 'CSS flexbox and grid as a reusable library.',
    body: 'stretch (Dec 2018) was the original flexbox crate. Taffy (Jun 2022) carries it forward. gpui, Blitz, floem, bevy_ui and Servo use it, and Slint is experimenting with it.',
    citation: CRATES
  },

  // ---- Accessibility ----
  {
    title: 'AccessKit: one accessibility layer',
    status: 0.8, statusNote: 'Widely adopted, one maintainer',
    year: 2021, month: 12, topic: 'Accessibility', kind: 'primitive',
    gesture: 'Screen-reader support shouldn’t be rebuilt per framework.',
    body: 'AccessKit (Dec 2021) is integrated by egui, Slint, Bevy, Xilem, Freya and Blitz, and in 2026 by gpui. iOS support landed in May 2026. The 2026 report flags that it has one active maintainer.',
    citation: CRATES + ' Integration list from the 2026 ecosystem report.',
    link: 'https://goldstrikearch.github.io/rust-gui-desktop-ecosystem-state/'
  },

  // ---- Frameworks ----
  {
    title: 'Druid, the retired bet',
    status: -0.9, statusNote: 'Sunset',
    year: 2018, month: 11, topic: 'Framework', kind: 'framework',
    gesture: 'The “official” Rust GUI effort taught the field what to reuse.',
    body: 'Druid (Nov 2018) with piet (2019) was the Linebender team’s first toolkit. It was sunset. kurbo (2018) and peniko (2024) survive as shared vector types.',
    citation: CRATES
  },
  {
    title: 'gpui: Zed’s framework',
    status: 0.4, statusNote: 'Powers Zed; small outside ecosystem',
    year: 2022, month: 6, topic: 'Framework', kind: 'framework',
    gesture: 'A GUI framework built to make an editor fast.',
    body: 'The `gpui` crate name dates to Jun 2022, but the 0.2.x public releases are recent (2025). gpui-component (Feb 2025) builds widgets on it. Zed moved its Linux renderer from Blade to wgpu in Feb 2026.',
    citation: CRATES + ' Check Zed’s own announcement for the real public date.'
  },
  {
    title: 'Makepad and Freya',
    status: 0.05, statusNote: 'Niche, active, pre-stable',
    year: 2022, month: 11, topic: 'Framework', kind: 'framework',
    gesture: 'Two more GPU-first takes on the same problem.',
    body: 'Makepad’s crate appeared Oct 2022, with its own shader-driven renderer. Freya (Nov 2022) uses Dioxus as its core with Skia underneath.',
    citation: CRATES
  },
  {
    title: 'Floem: from the Lapce editor',
    status: 0.0, statusNote: 'Niche, small user base',
    year: 2023, month: 11, topic: 'Framework', kind: 'framework',
    gesture: 'A reactive native toolkit pulled out of a code editor.',
    body: 'floem (Nov 2023) uses a winit fork and Taffy. The 2026 report lists it as AccessKit-free.',
    citation: CRATES
  },
  {
    title: 'Xilem: the Linebender rewrite',
    status: 0.1, statusNote: 'Promising, pre-1.0, few users',
    year: 2024, month: 5, topic: 'Framework', kind: 'framework',
    gesture: 'Reactive views on top of Masonry, Vello and Parley.',
    body: 'Xilem (May 2024) sits on Masonry (Nov 2022), Vello and Parley. It is still pre-1.0.',
    citation: CRATES
  },
  {
    title: 'Blitz and Dioxus Native',
    status: 0.0, statusNote: 'Early beta, tiny download base',
    year: 2025, month: 4, topic: 'Framework', kind: 'framework',
    gesture: 'A browser-less HTML/CSS renderer for native Dioxus.',
    body: 'blitz-dom and dioxus-native (Apr 2025) render HTML and CSS without a webview, using Taffy, Vello-family renderers and AccessKit.',
    citation: CRATES
  },

  {
    title: 'Rust 1.0',
    year: 2015, month: 5, topic: 'Language', kind: 'primitive', stable: true,
    status: 1.0, statusNote: 'The foundation',
    gesture: 'Nothing in this book exists before Rust promises not to break.',
    body: 'Rust 1.0 (15 May 2015) introduced the stability guarantee that let an ecosystem of libraries, windowing crates and eventually GUI toolkits build on a stable base.',
    citation: 'Rust project announcement, 15 May 2015. Date from memory; verify.'
  },
  {
    title: 'Tauri 1.0',
    year: 2022, month: 6, topic: 'Framework', kind: 'framework', stable: true,
    status: 0.9, statusNote: 'Superseded by 2.0, widely deployed',
    gesture: 'The first Rust GUI framework to declare itself stable.',
    body: 'Tauri 1.0 shipped 19 Jun 2022 after 9 months of betas and 4 months of release candidates. It was the first major Rust desktop framework to make a stability promise.',
    citation: 'Tauri 1.0 announcement, 19 Jun 2022.',
    link: 'https://en.wikipedia.org/wiki/Tauri_(software_framework)'
  },
  {
    title: 'Slint 1.0',
    year: 2023, month: 4, topic: 'Framework', kind: 'framework', stable: true,
    status: 0.6, statusNote: 'Stable API, commercially backed',
    gesture: 'A native toolkit that promised API stability.',
    body: 'Slint 1.0 (3 Apr 2023) committed to a stable Rust API and a stable `.slint` language, three years after the first `sixtyfps` crate.',
    citation: 'crates.io: slint 1.0.0, 2023-04-03.'
  },
  {
    title: 'Tauri 2.0',
    year: 2024, month: 10, topic: 'Framework', kind: 'framework', stable: true,
    status: 0.9, statusNote: 'Mobile and plugin system, mainstream',
    gesture: 'Desktop, then mobile, with a plugin system.',
    body: 'Tauri 2.0 (2 Oct 2024) added iOS and Android support and a new permissions and plugin model. A 3.0 alpha is already on crates.io in 2026.',
    citation: 'Tauri 2.0 announcement; crates.io: tauri 2.0.0, 2024-10-02.',
    link: 'https://v2.tauri.app/blog/tauri-20/'
  },
  {
    title: 'Makepad 1.0',
    year: 2025, month: 5, topic: 'Framework', kind: 'framework', stable: true,
    status: 0.3, statusNote: 'Stable label, small user base',
    gesture: 'A shader-driven toolkit declares 1.0.',
    body: 'Makepad’s widget crate reached 1.0.0 on 13 May 2025, more than two years after its first publish. Its download count remains small next to egui and Slint.',
    citation: 'crates.io: makepad-widgets 1.0.0, 2025-05-13.'
  },
  {
    title: 'WebGPU ships in every major browser',
    year: 2025, month: 11, topic: 'Rendering', kind: 'primitive', stable: true,
    status: 1.0, statusNote: 'Standardised target for wgpu on the web',
    gesture: 'wgpu’s browser backend stops being a bet.',
    body: 'Chrome and Edge had shipped WebGPU in version 113 (2023). Firefox 141 and Safari 26 completed the set in 2025. The standard is the model wgpu implements, so the web target became reliable for Rust UI frameworks.',
    citation: 'web.dev, “WebGPU is now supported in major browsers.”',
    link: 'https://web.dev/blog/webgpu-supported-major-browsers'
  },

  // ---- 2026 ----
  {
    title: 'Dioxus joins Cognition',
    year: 2026, month: 9, topic: 'Framework', kind: 'framework',
    status: 0.6, statusNote: 'Team now employed by Cognition; OSS continues',
    gesture: 'The biggest Rust GUI team now has a company paying for it.',
    body: 'On 10 Sep 2026 Dioxus Labs announced that its team is joining **Cognition**, the maker of the Devin coding agent, which already used Dioxus heavily (including a terminal renderer). The announcement frames this as the team joining, not an acquisition. The team says it will keep working on Dioxus, Blitz, Taffy and Subsecond, with more investment in Dioxus Native and Blitz. Taffy is the shared layout engine used across this book, so its maintainers’ employer matters beyond Dioxus. The post says nothing about licensing or governance changes.',
    citation: 'Dioxus Labs, “Joining Cognition,” 10 Sep 2026.',
    link: 'https://dioxuslabs.com/blog/joining-cognition/'
  },
  {
    title: 'Tauri gains a winit-gtk4 backend',
    status: 0.2, statusNote: 'Merged but tied to a beta winit',
    year: 2026, month: 7, topic: 'Windowing', kind: 'primitive',
    gesture: 'Linux’s GTK coupling is still the hard part.',
    body: 'A winit-gtk4 backend merged on 16 Jul 2026 against the winit 0.31 beta. Menus and tray icons remain tied to GTK on Linux.',
    citation: 'July–Aug 2026 ecosystem report.',
    link: 'https://goldstrikearch.github.io/rust-gui-desktop-ecosystem-state/'
  }
];

export const flat = raw
  .map((s) => ({ ...s, icon: TOPIC_ICONS[s.topic] ?? '•', pos: s.year + ((s.month ?? 1) - 1) / 12 }))
  .sort((a, b) => a.pos - b.pos)
  .map((s, i) => ({ ...s, num: String(i + 1).padStart(2, '0'), orderIndex: i }));

export const YEAR_MIN = Math.floor(Math.min(...flat.map((s) => s.pos)));
export const YEAR_MAX = Math.ceil(Math.max(...flat.map((s) => s.pos)));

export function next(num) {
  const i = flat.findIndex((s) => s.num === num);
  return i >= 0 && i < flat.length - 1 ? flat[i + 1] : null;
}

export function prev(num) {
  const i = flat.findIndex((s) => s.num === num);
  return i > 0 ? flat[i - 1] : null;
}
