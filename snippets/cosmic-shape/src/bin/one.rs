use cosmic_text::{Attrs, Buffer, FontSystem, Metrics, Shaping};

fn main() {
    let mut fonts = FontSystem::new(); // discovers system fonts
    let mut buffer = Buffer::new(&mut fonts, Metrics::new(16.0, 20.0));
    buffer.set_size(Some(200.0), None);
    buffer.set_text("Hello, שלום 👋", &Attrs::new(), Shaping::Advanced, None);
    buffer.shape_until_scroll(&mut fonts, false);

    for run in buffer.layout_runs() {
        println!("line {} is {:.1}px wide with {} glyphs", run.line_i, run.line_w, run.glyphs.len());
    }
}
