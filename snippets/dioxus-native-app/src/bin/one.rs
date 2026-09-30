use dioxus::prelude::*;

// Identical component code to the desktop/web Dioxus examples: only the
// launcher changes. No webview is involved; Blitz lays out and paints the DOM.
fn main() {
    dioxus_native::launch(app);
}

fn app() -> Element {
    let mut count = use_signal(|| 0);
    rsx! {
        div { style: "padding: 24px; font-family: sans-serif;",
            h1 { "Native Dioxus" }
            button { onclick: move |_| count += 1, "clicked {count} times" }
        }
    }
}
