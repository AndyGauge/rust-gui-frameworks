Rune is the most-watched new framework of 2026. It comes from **Anomaly**, the company behind the opencode coding agent. The repository opened on 22 October 2026 and had about 6,600 GitHub stars within three weeks, more than any other framework that appeared in 2026. It is a GPU-rendered toolkit for macOS, Windows, Linux and the web, and it is designed from the start to be driven by coding agents as well as by people.

::inline rust | label=examples/hello at rev cd83aff (illustrative, not compiled)
#[derive(Default)]
struct State {
    on: bool,
}

fn view(s: &State, _: &mut Cx<State>) -> El {
    let t = tok::theme(true);
    let label = text(if s.on { "Lights on" } else { "Lights off" })
        .font(type_::TITLE).color(t.text);
    let knob = el().key("knob").size(18.0, 18.0).radius(9.0).fill(t.knob)
        .absolute(if s.on { 18.0 } else { 2.0 }, 2.0);
    // Screen readers hear "Lights, switch, on".
    let switch = el().key("switch").role("switch").label("Lights").checked(s.on)
        .focusable(true).size(38.0, 22.0).radius(11.0)
        .fill(if s.on { tok::accent(0, true) } else { t.control_off })
        .motion(motion::SNAPPY)
        .on_press(|s: &mut State| s.on = !s.on)
        .child(knob);
    col().w_full().h_full().center().gap(16.0).fill(t.bg).child(label).child(switch)
}
::end

The model is Elm-shaped, like iced. The view is a plain function of state, and handlers get `&mut State` between frames. The difference is motion. When the view re-runs, Rune diffs it and moves each changed property along a spring, so the knob above glides to its new position instead of jumping. Motion is a core part of the design. Colours animate in OKLab, layout animates per edge, and there is a reduced-motion mode.

The same app runs without a window or a GPU, which is how an agent checks its own work. The repository's tests click the switch and step frames:

::inline rust | label=from the same example's tests (illustrative, not compiled)
let mut rt = Runtime::new(State::default(), view, (320.0, 200.0), env);
rt.frame(0.0);
let (x, y) = rt.find("switch").unwrap();
rt.pointer_down(x, y);
rt.pointer_up(x, y);
for _ in 0..6 { rt.frame(1.0 / 60.0); }
assert!(rt.state.on);
::end

Its `Cargo.toml` shows Rune taking the standard layers and replacing the parts above them. It uses winit **0.30.13**, wgpu **30.0.1** (with a patched `wgpu-hal` that every app must repeat), Taffy **0.15**, Parley **0.12** and AccessKit **0.25.2**. All of these are newer than the versions most frameworks in the version-skew table pin. On top of them, rendering is a single instanced "uber-shader" pipeline with one draw call per frame. Text is rasterised through ports of CoreGraphics's and DirectWrite's glyph pipelines, so it matches the platform's own text.

Two things set Rune apart from everything else in this book. It is **not on crates.io**, because the name `rune` belongs to an unrelated scripting language, so you pin a git rev of its `dev` branch. It also **does not take pull requests**. The README asks contributors to "have your agent open an issue", and says the team's own agents turn those around. The `dev` branch had over 2,000 commits after three weeks. Rune is MIT-licensed and pre-release.

This page is the one exception in the book: its code is quoted from the repository, not compiled. Building a git-only dependency that patches `wgpu-hal` was outside what the book's verification run was permitted to do.
