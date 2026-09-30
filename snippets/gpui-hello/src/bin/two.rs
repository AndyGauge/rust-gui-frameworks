use gpui::{App, Application, Context, Window, div, prelude::*, rgb};

// The same Tailwind-style vocabulary works for any layout. Under the hood,
// every `flex()` / `gap_2()` call becomes a Taffy style.
struct Toolbar;

impl Render for Toolbar {
    fn render(&mut self, _w: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .gap_2()
            .p_2()
            .bg(rgb(0x181825))
            .children(["File", "Edit", "View"].map(|name| {
                div().px_3().py_1().rounded_sm().hover(|s| s.bg(rgb(0x313244))).child(name)
            }))
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.open_window(Default::default(), |_, cx| cx.new(|_| Toolbar)).unwrap();
    });
}
