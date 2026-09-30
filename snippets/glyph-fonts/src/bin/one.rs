use ab_glyph::{Font, FontRef, PxScale, ScaleFont};
use book_error::{Context, Result};

fn main() -> Result<()> {
    // Three different failures, three different variants: a missing file
    // (Io), a corrupt font (Font), and a glyph with no outline (Missing).
    let bytes = std::fs::read("/System/Library/Fonts/Supplemental/Arial.ttf")?;
    let font = FontRef::try_from_slice(&bytes)?;
    let scaled = font.as_scaled(PxScale::from(32.0));

    // Glyph outlines in, coverage values out: no shaping, no layout.
    let glyph = font.glyph_id('R').with_scale(32.0);
    let outline = font.outline_glyph(glyph).context("font has no outline for 'R'")?;
    let bounds = outline.px_bounds();
    outline.draw(|_x, _y, coverage| {
        let _ = coverage; // 0.0..=1.0 alpha for one pixel
    });
    println!("R is {}x{} px, advance {}", bounds.width(), bounds.height(), scaled.h_advance(font.glyph_id('R')));
    Ok(())
}
