use wasm_bindgen::prelude::*;

// Call *into* JavaScript...
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(message: &str);
}

// ...and expose Rust *out* to it.
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    log(&format!("greeting {name}"));
    format!("Hello, {name}!")
}

fn main() {}
