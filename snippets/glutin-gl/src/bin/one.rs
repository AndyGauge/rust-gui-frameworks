use glutin::config::ConfigTemplateBuilder;

fn main() {
    // Describe the framebuffer you want; the platform picks the closest match.
    let template = ConfigTemplateBuilder::new()
        .with_alpha_size(8)
        .with_multisampling(4)
        .build();
    // `template` is later passed to Display::find_configs(template).
    let _ = template;
}
