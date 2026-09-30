use tiny_skia::{Color, FillRule, Paint, PathBuilder, Pixmap, Transform};

fn main() {
    let mut pixmap = Pixmap::new(200, 200).unwrap();
    pixmap.fill(Color::WHITE);

    let mut paint = Paint::default();
    paint.set_color_rgba8(50, 127, 150, 255);
    paint.anti_alias = true;

    let mut pb = PathBuilder::new();
    pb.move_to(100.0, 20.0);
    pb.line_to(180.0, 180.0);
    pb.line_to(20.0, 180.0);
    pb.close();
    let triangle = pb.finish().unwrap();

    pixmap.fill_path(&triangle, &paint, FillRule::Winding, Transform::identity(), None);
    pixmap.save_png("triangle.png").unwrap();
}
