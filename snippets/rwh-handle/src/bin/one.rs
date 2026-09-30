use raw_window_handle::{HasWindowHandle, RawWindowHandle};

/// What kind of native object is behind this window?
fn describe(window: &impl HasWindowHandle) -> &'static str {
    match window.window_handle().unwrap().as_raw() {
        RawWindowHandle::AppKit(_) => "NSView (macOS)",
        RawWindowHandle::Win32(_) => "HWND (Windows)",
        RawWindowHandle::Xlib(_) | RawWindowHandle::Xcb(_) => "X11 window",
        RawWindowHandle::Wayland(_) => "wl_surface (Wayland)",
        RawWindowHandle::Web(_) => "canvas element (browser)",
        _ => "something else",
    }
}

fn main() {
    // winit's Window implements HasWindowHandle, so this just works:
    let _ = |w: &winit::window::Window| describe(w);
}
