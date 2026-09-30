Earlier drafts of these snippets would have panicked on the first problem: no GPU, a missing font, a window the operating system refused to create. Production code treats those as ordinary outcomes. Every snippet in the book now avoids `unwrap()` and `expect()`. The difficulty is that every framework reports failure differently: winit has `EventLoopError` and `OsError`, wgpu has separate adapter and device errors, Taffy has `TaffyError`, ab_glyph has `InvalidFont`, and tiny-skia returns a bare `Option`. An application wants *one* error type that all of them convert into, so `?` works everywhere. That is the small `book-error` crate every snippet depends on. Its core is an enum with a variant per source:

::inline rust | label=book-error/src/error.rs (abridged, illustrative)
pub enum Error {
    Io(std::io::Error),
    Missing(String),                                   // an Option that was None
    Context { context: String, source: Box<Error> },   // "while doing X..."
    Other(Box<dyn std::error::Error + Send + Sync>),   // anything unlisted

    #[cfg(feature = "winit")]
    EventLoop(winit::error::EventLoopError),
    #[cfg(feature = "wgpu")]
    Adapter(wgpu::RequestAdapterError),
    // ...one variant per framework, behind a Cargo feature of the same name
}
::end

The `From` impls are what make the `?` operator convert automatically. A small macro keeps each one to a line, and each is behind its framework's feature, so a snippet only compiles the conversions it uses.

::code book-error/src/convert.rs

Conversion alone loses *what you were doing*. The `Context` trait adds that, and also does a second job: it turns an `Option` into an error. Library constructors such as tiny-skia's `Pixmap::new` return `None` on failure, and `.context("could not allocate a 200x200 pixmap")?` gives that `None` a message.

::code book-error/src/context.rs

Each layer adds what it was trying to do, and the original cause is kept. The demo below fails on purpose, and the output is what it really printed.

::code book-error/src/bin/demo.rs

Some places cannot use `?` at all, and the snippets show how each is handled. **Event-loop callbacks** return `()`, so the winit examples store the error in the app struct and exit the loop, and `main` returns it afterwards. **Entry points** that cannot return a `Result`, like Tauri's mobile entry or gpui's startup closure, report the error and exit at that one boundary. **WebAssembly** turns an `Error` into a JavaScript exception through a `From<Error> for JsValue` impl. **Build scripts** return `Box<dyn Error>`, which Cargo prints.

Two lessons from building it. Cargo resolves every optional dependency when it writes a lockfile, so Druid (gtk-rs 0.16 bindings) and Tauri (gtk-rs 0.18) could not both be features of one crate: they link the same native library. Druid therefore goes through `Error::other`. And raw-window-handle's `HandleError` only implements `std::error::Error` with its `std` feature on, which several snippets got by accident through winit or wgpu, until the glutin one did not. Still, `unwrap()` is not banned everywhere: it is fine in tests and for states that are genuinely impossible. A window library failing to start is not one of those.
