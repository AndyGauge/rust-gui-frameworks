use book_error::{Context, Result};
use tauri::Manager;

// A command: Rust code the web frontend can call by name.
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {name}! (from Rust)")
}

fn try_run() -> Result<()> {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            // The webview window is a native window too: it exposes the same
            // raw handles any renderer would want. Setup hooks return a boxed
            // error, and our `Error` converts into one, so `?` just works.
            use raw_window_handle::HasWindowHandle;
            let window = app.get_webview_window("main").context("no window labelled `main`")?;
            let _handle = window.window_handle()?;
            Ok(())
        })
        .run(tauri::generate_context!())?;
    Ok(())
}

// Mobile entry points cannot return a Result, so this is the one place an
// error is finally reported and turned into a non-zero exit.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    if let Err(err) = try_run() {
        eprintln!("{}", err.report());
        std::process::exit(1);
    }
}
