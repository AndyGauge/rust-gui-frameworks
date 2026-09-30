fn main() {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());

    // Ask for the best GPU the OS and drivers will give us.
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default()))
        .expect("no GPU adapter found");
    let info = adapter.get_info();
    println!("{} via {:?}", info.name, info.backend);

    // A device is the logical connection; the queue submits work to it.
    let (_device, _queue) =
        pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default())).unwrap();
}
