use book_error::{Context, Error};
use wasm_bindgen::prelude::*;

// Runs automatically when the .wasm module is loaded in the page. A returned
// error becomes a JavaScript exception (see `From<Error> for JsValue`).
#[wasm_bindgen(start)]
fn run() -> Result<(), Error> {
    let document = web_sys::window()
        .context("no global `window` (not running in a browser?)")?
        .document()
        .context("window has no document")?;
    let p = document.create_element("p")?;
    p.set_text_content(Some("Hello from Rust!"));
    document.body().context("document has no <body>")?.append_child(&p)?;
    Ok(())
}

fn main() {}
