use blinc_app::prelude::*;
use blinc_app::windowed::{WindowedApp, WindowedContext};
use book_error::{Error, Result};

// A builder chain that reads like Tailwind: div().flex_col().gap(..).
// The closure is the whole UI, and the same function runs on the web.
fn ui(ctx: &mut WindowedContext) -> impl ElementBuilder {
    div()
        .w(ctx.width)
        .h(ctx.height)
        .bg(Color::rgb(0.1, 0.1, 0.15))
        .justify_center()
        .items_center()
        .child(text("Hello Blinc!").size(48.0).color(Color::WHITE))
}

fn main() -> Result<()> {
    WindowedApp::run(WindowConfig::default(), ui).map_err(Error::other)
}
