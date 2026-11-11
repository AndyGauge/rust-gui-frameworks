use book_error::{Error, Result};
use zgui::prelude::*;

// Components run once. The closure around `count.get()` is a "reactive
// hole": it subscribes to `count` and rewrites one text node when it changes.
#[component]
fn Counter() -> impl IntoView {
    let count = RwSignal::new(0);

    view! {
        column(class = "counter") {
            text(class = "value") {{move || count.get().to_string()}}
            control(
                tabindex = Focus::Sequential,
                on:click = move |_| count.update(|n| *n += 1)
            ) {
                "Increment"
            }
        }
    }
}

// Real CSS, cascaded by Stylo (Servo's style engine), not a Rust style DSL.
const STYLE: &str = css!(
    ".counter { gap: 12px; align-items: center; }
    .value { font-size: 48px; font-weight: 700; }
    control { padding: 8px 16px; border-radius: 8px; background: #356df3; }
    control:hover { background: #477df8; }"
);

fn main() -> Result<()> {
    app()
        .with_title("Counter")
        .with_stylesheet(STYLE)
        .run(|| view! { Counter() })
        .map_err(Error::other)
}
