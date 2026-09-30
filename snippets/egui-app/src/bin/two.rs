use eframe::egui;

// egui itself knows nothing about windows or GPUs. It turns your UI into
// shapes and hands them to whatever integration is driving it.
fn main() {
    let ctx = egui::Context::default();
    let mut output = ctx.run_ui(egui::RawInput::default(), |ui| {
        ui.label("hello, headless egui");
    });

    let meshes = ctx.tessellate(std::mem::take(&mut output.shapes), output.pixels_per_point);
    println!("{} meshes, {} texture updates", meshes.len(), output.textures_delta.set.len());

    // A real backend (egui-wgpu, glow) uploads these font/image textures to the GPU.
    output.textures_delta.clear();
}
