use book_error::{Error, Result};
use wasm_bindgen::prelude::*;

// The same wgpu code as on desktop; in a browser it talks to navigator.gpu.
// A failure becomes a rejected JavaScript promise rather than a panic.
#[wasm_bindgen]
pub async fn gpu_name() -> Result<String, Error> {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await?;
    Ok(adapter.get_info().name)
}
