use blinc_app::prelude::*;
use blinc_app::windowed::{WindowedApp, WindowedContext};
use book_error::{Error, Result};

// State is a signal keyed by its call site. `stateful` re-renders only this
// subtree, and only when a signal listed in `deps` changes.
fn counter(ctx: &WindowedContext) -> impl ElementBuilder {
    let count = ctx.use_state(0i32);
    stateful::<NoState>()
        .deps([count.signal_id()])
        .on_state(move |_| {
            div()
                .p(16.0)
                .bg(Color::rgb(0.2, 0.3, 0.8))
                .child(text(&format!("Clicks: {}", count.get())).color(Color::WHITE))
                .on_click({
                    let count = count.clone();
                    move |_| count.update(|n| n + 1)
                })
        })
}

fn main() -> Result<()> {
    WindowedApp::run(WindowConfig::default(), |ctx| {
        div().w(ctx.width).h(ctx.height).justify_center().items_center().child(counter(ctx))
    })
    .map_err(Error::other)
}
