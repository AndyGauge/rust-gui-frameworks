use dioxus::prelude::*;

fn main() {
    dioxus_native::launch(app);
}

// Blitz resolves real CSS (via Stylo, the engine from Firefox), lays it out with
// Taffy, shapes text with Parley and paints with a Vello-family renderer.
fn app() -> Element {
    rsx! {
        div {
            style: "display: flex; gap: 12px; padding: 20px;",
            div { style: "flex: 1; background: #e5484d; height: 80px; border-radius: 8px;" }
            div { style: "flex: 2; background: #30a46c; height: 80px; border-radius: 8px;" }
        }
    }
}
