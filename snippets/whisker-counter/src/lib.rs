pub mod list;
use whisker::prelude::*;

#[whisker::main]
fn app() -> Element {
    render! {
        Counter
    }
}

#[component]
fn counter() -> Element {
    let count = signal(0);
    render! {
        View(style: css!(
            flex_grow: 1.0,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            gap: px(12),
            background_color: Color::hex(0x0B0B0F),
        )) {
            Text(
                value: computed(move || format!("Count: {}", count.get())),
                style: css!(color: Color::hex(0xFFFFFF), font_size: px(28)),
            )
            View(
                style: css!(
                    padding: (px(10), px(20)),
                    border_radius: px(10),
                    background_color: Color::hex(0x7C5CFF),
                ),
                on_tap: move |_| count.set(count.get() + 1),
            ) {
                Text(value: "+1", style: css!(color: Color::hex(0xFFFFFF)))
            }
        }
    }
}
