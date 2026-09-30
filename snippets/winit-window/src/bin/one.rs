use book_error::{Error, Result};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

#[derive(Default)]
struct App {
    window: Option<Window>,
    // Handler methods return `()`, so they cannot use `?`. Stash the failure
    // here and let `main` report it after the loop exits.
    error: Option<Error>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attrs = Window::default_attributes().with_title("hello winit");
        match event_loop.create_window(attrs) {
            Ok(window) => self.window = Some(window),
            Err(e) => {
                self.error = Some(e.into());
                event_loop.exit();
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        if let WindowEvent::CloseRequested = event {
            event_loop.exit();
        }
    }
}

fn main() -> Result<()> {
    let mut app = App::default();
    EventLoop::new()?.run_app(&mut app)?;
    app.error.take().map_or(Ok(()), Err)
}
