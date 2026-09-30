use tauri::Manager;

// A command: Rust code the web frontend can call by name.
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {name}! (from Rust)")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            // The webview window is a native window too: it exposes the same
            // raw handles any renderer would want.
            use raw_window_handle::HasWindowHandle;
            let window = app.get_webview_window("main").unwrap();
            let _handle = window.window_handle();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
