use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let clicks = use_state(|| 0);
    let onclick = {
        let clicks = clicks.clone();
        Callback::from(move |_| clicks.set(*clicks + 1))
    };
    html! { <button {onclick}>{ format!("clicked {} times", *clicks) }</button> }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
