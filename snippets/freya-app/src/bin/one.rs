use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

fn app() -> impl IntoElement {
    let mut count = use_state(|| 0);

    rect()
        .expanded()
        .center()
        .spacing(8.0)
        .child(format!("clicked {} times", count.read()))
        .child(
            Button::new()
                .on_press(move |_| *count.write() += 1)
                .child("Increase"),
        )
}
