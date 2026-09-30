use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

/// This bound is the entire contract between a window library and a renderer.
/// wgpu, glutin, softbuffer and Slint's backends all ask for exactly this.
trait RenderTarget: HasWindowHandle + HasDisplayHandle {}
impl<T: HasWindowHandle + HasDisplayHandle> RenderTarget for T {}

fn create_surface(_target: &dyn RenderTarget) {
    // ...hand the raw handles to Metal / Vulkan / D3D / WebGPU here...
}

fn main() {
    // winit's Window satisfies it, so does tao's, and so does any test double.
    let _: fn(&winit::window::Window) = |w| create_surface(w);
}
