use book_error::Error;
use gpui::{App, Application, Bounds, Context, Window, WindowBounds, WindowOptions, div, prelude::*, px, rgb, size};

struct Counter {
    count: u32,
}

impl Render for Counter {
    // Called whenever the entity is marked dirty; returns a tree of styled divs
    // using Tailwind-like methods. Taffy lays it out; GPUI paints it.
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .size_full()
            .justify_center()
            .items_center()
            .bg(rgb(0x1e1e2e))
            .text_color(rgb(0xffffff))
            .child(
                div()
                    .id("inc")
                    .p_4()
                    .bg(rgb(0x45475a))
                    .rounded_md()
                    .child(format!("clicked {} times", self.count))
                    .on_click(cx.listener(|this, _event, _window, cx| {
                        this.count += 1;
                        cx.notify(); // tell GPUI this view needs to re-render
                    })),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(400.), px(300.)), cx);
        let options = WindowOptions { window_bounds: Some(WindowBounds::Windowed(bounds)), ..Default::default() };
        // The startup closure returns (), so report the failure and quit the app.
        if let Err(e) = cx.open_window(options, |_, cx| cx.new(|_| Counter { count: 0 })) {
            eprintln!("{}", Error::other(e).report());
            cx.quit();
            return;
        }
        cx.activate(true);
    });
}
