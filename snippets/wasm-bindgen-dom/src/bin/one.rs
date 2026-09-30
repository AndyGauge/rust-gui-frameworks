use wasm_bindgen::prelude::*;

// Runs automatically when the .wasm module is loaded in the page.
#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let p = document.create_element("p")?;
    p.set_text_content(Some("Hello from Rust!"));
    document.body().unwrap().append_child(&p)?;
    Ok(())
}

fn main() {}
