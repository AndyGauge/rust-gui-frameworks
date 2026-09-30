use book_error::{Context, Error, Result};
use tiny_skia::{Color, GradientStop, LinearGradient, Paint, Pixmap, Point, Rect, SpreadMode, Transform};

fn main() -> Result<()> {
    let mut pixmap = Pixmap::new(200, 100).context("could not allocate a 200x100 pixmap")?;
    pixmap.fill(Color::from_rgba8(240, 240, 240, 255));

    let mut paint = Paint::default();
    paint.shader = LinearGradient::new(
        Point::from_xy(0.0, 0.0),
        Point::from_xy(200.0, 0.0),
        vec![
            GradientStop::new(0.0, Color::from_rgba8(255, 80, 80, 255)),
            GradientStop::new(1.0, Color::from_rgba8(80, 80, 255, 255)),
        ],
        SpreadMode::Pad,
        Transform::identity(),
    )
    .context("degenerate gradient (start and end points coincide)")?;

    // A software rasterizer: no GPU, no window. This is what resvg and
    // CPU fallbacks use to turn vector shapes into pixels.
    let rect = Rect::from_xywh(10.0, 10.0, 180.0, 80.0).context("rectangle has non-finite size")?;
    pixmap.fill_rect(rect, &paint, Transform::identity(), None);
    pixmap.save_png("gradient.png").map_err(Error::other)?;
    Ok(())
}
