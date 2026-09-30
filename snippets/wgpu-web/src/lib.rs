use wasm_bindgen::prelude::*;

// The same wgpu code as on desktop; in a browser it talks to navigator.gpu.
#[wasm_bindgen]
pub async fn gpu_name() -> String {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());
    match instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await {
        Ok(adapter) => adapter.get_info().name,
        Err(_) => "no WebGPU in this browser".to_string(),
    }
}
