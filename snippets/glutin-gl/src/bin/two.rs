#[cfg(target_os = "macos")]
fn main() -> book_error::Result<()> {
    use glutin::display::{Display, DisplayApiPreference, GlDisplay};
    use raw_window_handle::{AppKitDisplayHandle, RawDisplayHandle};

    // glutin never creates windows. It is handed a raw display handle (the
    // same one winit exposes) and gives back an OpenGL display for it.
    let raw = RawDisplayHandle::AppKit(AppKitDisplayHandle::new());
    // SAFETY: the handle describes this process's own AppKit display.
    let display = unsafe { Display::new(raw, DisplayApiPreference::Cgl) }?;
    println!("GL display: {}", display.version_string());
    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn main() {}
