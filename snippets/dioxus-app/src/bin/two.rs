use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn Item(label: String, done: bool) -> Element {
    rsx! { li { class: if done { "done" }, "{label}" } }
}

#[component]
fn App() -> Element {
    let todos = use_signal(|| vec![("write book", true), ("ship it", true), ("add code", false)]);
    rsx! {
        ul {
            for (label, done) in todos.read().iter() {
                Item { label: label.to_string(), done: *done }
            }
        }
    }
}
