fn main() {
    // Metal on macOS, Vulkan/GL on Linux, D3D12 on Windows, WebGPU in a browser.
    let mut desc = wgpu::InstanceDescriptor::new_without_display_handle();
    desc.backends = wgpu::Backends::PRIMARY;
    let instance = wgpu::Instance::new(desc);

    // No usable GPU? Ask for a software adapter instead: the route a
    // "software rendering" fallback takes.
    let options = wgpu::RequestAdapterOptions {
        force_fallback_adapter: true,
        ..Default::default()
    };
    match pollster::block_on(instance.request_adapter(&options)) {
        Ok(a) => println!("fallback: {}", a.get_info().name),
        Err(e) => println!("no software adapter here: {e}"),
    }
}
