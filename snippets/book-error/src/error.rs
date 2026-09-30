//! The error enum: a variant per source, plus `Display` and `source()`.

use std::error::Error as StdError;
use std::fmt;

/// `Result` with our [`Error`] as the default error type.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Anything that implements `std::error::Error` can be boxed into this.
type Source = Box<dyn StdError + Send + Sync + 'static>;

#[non_exhaustive]
pub enum Error {
    /// Filesystem and other I/O failures.
    Io(std::io::Error),
    /// A value we needed was absent (an `Option` that was `None`).
    Missing(String),
    /// Something went wrong *while* doing something; keeps the cause.
    Context { context: String, source: Box<Error> },
    /// Escape hatch for error types this crate has no variant for.
    Other(Source),

    // One variant per framework, compiled only when its feature is on.
    #[cfg(feature = "semver")]
    Semver(semver::Error),
    #[cfg(feature = "glutin")]
    Gl(glutin::error::Error),
    #[cfg(feature = "winit")]
    EventLoop(winit::error::EventLoopError),
    #[cfg(feature = "winit")]
    Os(winit::error::OsError),
    #[cfg(feature = "raw-window-handle")]
    Handle(raw_window_handle::HandleError),
    #[cfg(feature = "wgpu")]
    Adapter(wgpu::RequestAdapterError),
    #[cfg(feature = "wgpu")]
    Device(wgpu::RequestDeviceError),
    #[cfg(feature = "taffy")]
    Layout(taffy::TaffyError),
    #[cfg(feature = "vello")]
    Render(vello::Error),
    #[cfg(feature = "ab-glyph")]
    Font(ab_glyph::InvalidFont),
    #[cfg(feature = "slint")]
    Platform(slint::PlatformError),
    #[cfg(feature = "tauri")]
    Tauri(tauri::Error),
    #[cfg(feature = "iced")]
    Iced(iced::Error),
    #[cfg(feature = "eframe")]
    Eframe(eframe::Error),
    /// A JavaScript exception. `JsValue` is not `Send`, so keep its text.
    #[cfg(feature = "wasm")]
    Js(String),
}

impl Error {
    /// Wrap any other error type.
    pub fn other(error: impl Into<Source>) -> Self {
        Error::Other(error.into())
    }

    /// The error and its whole chain of causes, one per line.
    pub fn report(&self) -> String {
        let mut out = self.to_string();
        let mut cause = self.source();
        while let Some(e) = cause {
            out.push_str(&format!("\n  caused by: {e}"));
            cause = e.source();
        }
        out
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "I/O error: {e}"),
            Error::Missing(what) => write!(f, "{what}"),
            Error::Context { context, .. } => write!(f, "{context}"),
            Error::Other(e) => write!(f, "{e}"),
            #[cfg(feature = "semver")]
            Error::Semver(e) => write!(f, "invalid version: {e}"),
            #[cfg(feature = "glutin")]
            Error::Gl(e) => write!(f, "OpenGL error: {e}"),
            #[cfg(feature = "winit")]
            Error::EventLoop(e) => write!(f, "event loop error: {e}"),
            #[cfg(feature = "winit")]
            Error::Os(e) => write!(f, "window system error: {e}"),
            #[cfg(feature = "raw-window-handle")]
            Error::Handle(e) => write!(f, "window handle unavailable: {e}"),
            #[cfg(feature = "wgpu")]
            Error::Adapter(e) => write!(f, "no suitable GPU adapter: {e}"),
            #[cfg(feature = "wgpu")]
            Error::Device(e) => write!(f, "could not open GPU device: {e}"),
            #[cfg(feature = "taffy")]
            Error::Layout(e) => write!(f, "layout error: {e}"),
            #[cfg(feature = "vello")]
            Error::Render(e) => write!(f, "render error: {e}"),
            #[cfg(feature = "ab-glyph")]
            Error::Font(e) => write!(f, "invalid font: {e}"),
            #[cfg(feature = "slint")]
            Error::Platform(e) => write!(f, "platform error: {e}"),
            #[cfg(feature = "tauri")]
            Error::Tauri(e) => write!(f, "tauri error: {e}"),
            #[cfg(feature = "iced")]
            Error::Iced(e) => write!(f, "iced error: {e}"),
            #[cfg(feature = "eframe")]
            Error::Eframe(e) => write!(f, "eframe error: {e}"),
            #[cfg(feature = "wasm")]
            Error::Js(e) => write!(f, "JavaScript error: {e}"),
        }
    }
}

// `main` returning `Err` prints the Debug form, so make that the readable report.
impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.report())
    }
}

impl StdError for Error {
    // Wrapper variants already print their inner message in `Display`, so they
    // hand back the inner error's *own* source. That keeps `report()` free of
    // repeated lines. Only `Context` points at a whole other `Error`.
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Io(e) => e.source(),
            Error::Missing(_) => None,
            Error::Context { source, .. } => Some(source.as_ref()),
            Error::Other(e) => e.source(),
            #[cfg(feature = "semver")]
            Error::Semver(e) => e.source(),
            #[cfg(feature = "glutin")]
            Error::Gl(e) => e.source(),
            #[cfg(feature = "winit")]
            Error::EventLoop(e) => e.source(),
            #[cfg(feature = "winit")]
            Error::Os(e) => e.source(),
            #[cfg(feature = "raw-window-handle")]
            Error::Handle(e) => e.source(),
            #[cfg(feature = "wgpu")]
            Error::Adapter(e) => e.source(),
            #[cfg(feature = "wgpu")]
            Error::Device(e) => e.source(),
            #[cfg(feature = "taffy")]
            Error::Layout(e) => e.source(),
            #[cfg(feature = "vello")]
            Error::Render(e) => e.source(),
            #[cfg(feature = "ab-glyph")]
            Error::Font(e) => e.source(),
            #[cfg(feature = "slint")]
            Error::Platform(e) => e.source(),
            #[cfg(feature = "tauri")]
            Error::Tauri(e) => e.source(),
            #[cfg(feature = "iced")]
            Error::Iced(e) => e.source(),
            #[cfg(feature = "eframe")]
            Error::Eframe(e) => e.source(),
            #[cfg(feature = "wasm")]
            Error::Js(_) => None,
        }
    }
}
