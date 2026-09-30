use rusttype::{point, Font, Scale};

// rusttype (2016) is deprecated; ab_glyph is its maintained successor.
// The shape of the API shows what "just rasterize text" looked like.
fn main() {
    let bytes = std::fs::read("/System/Library/Fonts/Supplemental/Arial.ttf").unwrap();
    let font = Font::try_from_vec(bytes).unwrap();

    let glyphs: Vec<_> = font.layout("Hi", Scale::uniform(32.0), point(0.0, 32.0)).collect();
    for g in &glyphs {
        if let Some(bb) = g.pixel_bounding_box() {
            println!("glyph at {}..{}", bb.min.x, bb.max.x);
        }
    }
}
