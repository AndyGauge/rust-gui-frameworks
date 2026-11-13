//! A keyed, virtualised list. Tapping a chip filters it; only the rows whose
//! keys change are created or released.
use whisker::prelude::*;

const LAYERS: [&str; 4] = ["All", "Windowing", "Rendering", "Layout"];
const CRATES: [(&str, usize); 6] =
    [("winit", 1), ("glutin", 1), ("wgpu", 2), ("vello", 2), ("taffy", 3), ("morphorm", 3)];

#[component]
pub fn crate_list() -> Element {
    let layer = signal(0usize);
    render! {
        View(style: Css::new().flex_grow(1.0).display(Display::Flex).flex_direction(FlexDirection::Column)) {
            View(style: Css::new().display(Display::Flex).flex_direction(FlexDirection::Row).gap(px(8)).padding(px(16))) {
                ForEach(each: || (0..LAYERS.len()).collect::<Vec<_>>(), key: |i: &usize| *i, children: move |i: ReadSignal<usize>| {
                    let i = i.get_untracked();
                    render! { Chip(index: i, layer: layer) }
                })
            }
            List(
                each: move || (0..CRATES.len()).filter(|&c| layer.get() == 0 || CRATES[c].1 == layer.get()).collect::<Vec<_>>(),
                key: |c: &usize| *c,
                children: |c: ReadSignal<usize>| render! { Row(name: CRATES[c.get_untracked()].0) },
                style: Css::new().flex_grow(1.0),
            )
        }
    }
}

#[component]
fn chip(index: usize, layer: RwSignal<usize>) -> Element {
    render! {
        View(
            on_tap: move |_| layer.set(index),
            style: computed(move || Css::new().padding((px(6), px(12))).border_radius(px(16))
                .background_color(Color::hex(if layer.get() == index { 0x7C5CFF } else { 0x2A2A33 }))),
        ) {
            Text(value: LAYERS[index], style: css!(color: Color::hex(0xFFFFFF), font_size: px(13)))
        }
    }
}

#[component]
fn row(name: &'static str) -> Element {
    render! {
        View(style: Css::new().padding((px(12), px(16)))) {
            Text(value: name, style: css!(color: Color::hex(0xFFFFFF), font_size: px(17)))
        }
    }
}
