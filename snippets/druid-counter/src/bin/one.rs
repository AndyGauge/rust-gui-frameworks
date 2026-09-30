use druid::widget::{Button, Flex, Label};
use druid::{AppLauncher, Widget, WidgetExt, WindowDesc};

fn ui() -> impl Widget<u32> {
    let label = Label::dynamic(|n: &u32, _| format!("count: {n}")).padding(5.0).center();
    let button = Button::new("increment").on_click(|_ctx, n, _env| *n += 1).padding(5.0);
    Flex::column().with_child(label).with_child(button)
}

fn main() {
    AppLauncher::with_window(WindowDesc::new(ui()).title("druid"))
        .launch(0)
        .unwrap();
}
