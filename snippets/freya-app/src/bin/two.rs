use freya::animation::*;
use freya::prelude::*;

fn main() {
    launch(LaunchConfig::new().with_window(WindowConfig::new(app)))
}

// Freya 0.4 builds elements with plain Rust method chains, no macro and
// no longer any dependency on Dioxus.
fn app() -> impl IntoElement {
    let mut animation = use_animation(|_| AnimColor::new((246, 240, 240), (205, 86, 86)).time(400));

    rect()
        .background(&*animation.read())
        .expanded()
        .center()
        .child(
            Button::new()
                .on_press(move |_| animation.start())
                .child("Start"),
        )
}
