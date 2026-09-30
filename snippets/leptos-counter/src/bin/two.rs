use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(1);
    // Derived values re-run only when the signals they read change;
    // there is no virtual DOM diff.
    let doubled = move || count.get() * 2;
    let label = Memo::new(move |_| if count.get() % 2 == 0 { "even" } else { "odd" });
    view! {
        <button on:click=move |_| set_count.update(|n| *n += 1)>"+1"</button>
        <p>{count} " doubled is " {doubled} " and is " {label}</p>
    }
}

fn main() {
    leptos::mount::mount_to_body(App);
}
