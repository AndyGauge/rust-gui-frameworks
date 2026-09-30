use ab_glyph::{Font, FontRef, PxScale, ScaleFont};

fn main() {
    let bytes = std::fs::read("/System/Library/Fonts/Supplemental/Arial.ttf").unwrap();
    let font = FontRef::try_from_slice(&bytes).unwrap();
    let scaled = font.as_scaled(PxScale::from(32.0));

    // Glyph outlines in, coverage values out: no shaping, no layout.
    let glyph = font.glyph_id('R').with_scale(32.0);
    let outline = font.outline_glyph(glyph).unwrap();
    let bounds = outline.px_bounds();
    outline.draw(|_x, _y, coverage| {
        let _ = coverage; // 0.0..=1.0 alpha for one pixel
    });
    println!("R is {}x{} px, advance {}", bounds.width(), bounds.height(), scaled.h_advance(font.glyph_id('R')));
}
