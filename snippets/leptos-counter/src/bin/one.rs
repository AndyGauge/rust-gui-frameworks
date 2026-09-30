use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    view! {
        <button on:click=move |_| *set_count.write() += 1>
            "clicked " {count} " times"
        </button>
    }
}

fn main() {
    leptos::mount::mount_to_body(App);
}
