use tiny_skia::{Color, LinearGradient, Paint, Point, Pixmap, Rect, SpreadMode, Transform, GradientStop};

fn main() {
    let mut pixmap = Pixmap::new(200, 100).unwrap();
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
    .unwrap();

    // A software rasterizer: no GPU, no window. This is what resvg and
    // CPU fallbacks use to turn vector shapes into pixels.
    let rect = Rect::from_xywh(10.0, 10.0, 180.0, 80.0).unwrap();
    pixmap.fill_rect(rect, &paint, Transform::identity(), None);
    pixmap.save_png("gradient.png").unwrap();
}
