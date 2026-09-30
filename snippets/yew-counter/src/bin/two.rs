use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct GreetingProps {
    name: AttrValue,
}

#[function_component]
fn Greeting(props: &GreetingProps) -> Html {
    html! { <h1>{ format!("Hello, {}!", props.name) }</h1> }
}

#[function_component]
fn App() -> Html {
    // Components compose like HTML; props are checked at compile time.
    html! { <><Greeting name="browser" /><Greeting name="WASM" /></> }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
