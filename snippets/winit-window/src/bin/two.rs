use book_error::{Error, Result};
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

struct App {
    window: Option<Window>,
    size: PhysicalSize<u32>,
    error: Option<Error>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        match event_loop.create_window(Window::default_attributes()) {
            Ok(window) => self.window = Some(window),
            Err(e) => {
                self.error = Some(e.into());
                event_loop.exit();
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            // A renderer resizes its swapchain here; a UI toolkit re-runs layout.
            WindowEvent::Resized(size) => self.size = size,
            // The toolkit paints here, then asks for the next frame if animating.
            WindowEvent::RedrawRequested => {
                if let Some(window) = &self.window {
                    let scale = window.scale_factor();
                    println!("paint {}x{} @ {scale}x", self.size.width, self.size.height);
                }
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => {}
        }
    }
}

fn main() -> Result<()> {
    let mut app = App { window: None, size: PhysicalSize::new(0, 0), error: None };
    EventLoop::new()?.run_app(&mut app)?;
    app.error.take().map_or(Ok(()), Err)
}
