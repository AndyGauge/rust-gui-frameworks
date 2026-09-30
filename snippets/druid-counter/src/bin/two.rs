use druid::widget::{Button, Flex, Label};
use druid::{AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc};

// Druid's model: your state is `Data`, and `Lens`es focus widgets onto fields.
// Xilem's map_state is the descendant of this idea.
#[derive(Clone, Data, Lens)]
struct State {
    likes: u32,
    dislikes: u32,
}

fn counter(name: &'static str) -> impl Widget<u32> {
    Flex::row()
        .with_child(Label::dynamic(move |n: &u32, _| format!("{name}: {n}")))
        .with_child(Button::new("+").on_click(|_, n, _| *n += 1))
}

fn ui() -> impl Widget<State> {
    Flex::column()
        .with_child(counter("likes").lens(State::likes))
        .with_child(counter("dislikes").lens(State::dislikes))
}

fn main() {
    AppLauncher::with_window(WindowDesc::new(ui()))
        .launch(State { likes: 0, dislikes: 0 })
        .unwrap();
}
