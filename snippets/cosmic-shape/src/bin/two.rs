use cosmic_text::{Attrs, Buffer, Family, FontSystem, Metrics, Shaping, Weight};

fn main() {
    let mut fonts = FontSystem::new();
    let mut buffer = Buffer::new(&mut fonts, Metrics::new(14.0, 18.0));
    buffer.set_size(Some(120.0), None);

    // Rich text: each span carries its own attributes.
    let base = Attrs::new().family(Family::SansSerif);
    let bold = base.clone().weight(Weight::BOLD);
    buffer.set_rich_text(
        [("Bold ", bold), ("and plain text that wraps at 120px", base.clone())],
        &base,
        Shaping::Advanced,
        None,
    );
    buffer.shape_until_scroll(&mut fonts, false);
    println!("wrapped into {} lines", buffer.layout_runs().count());
}
