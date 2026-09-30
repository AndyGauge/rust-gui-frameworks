use book_error::Result;
use vello::wgpu;
use vello::{AaConfig, RenderParams, Renderer, RendererOptions, Scene};

// Turning a Scene into pixels needs a wgpu device and a target texture.
fn render(device: &wgpu::Device, queue: &wgpu::Queue, scene: &Scene, target: &wgpu::TextureView) -> Result<()> {
    let mut renderer = Renderer::new(device, RendererOptions::default())?;
    let params = RenderParams {
        base_color: vello::peniko::Color::WHITE,
        width: 512,
        height: 512,
        antialiasing_method: AaConfig::Area,
    };
    renderer.render_to_texture(device, queue, scene, target, &params)?;
    Ok(())
}

fn main() {
    let _ = render;
}
