use vello::kurbo::{Affine, Circle, RoundedRect};
use vello::peniko::{Color, Fill};
use vello::Scene;

fn main() {
    let mut scene = Scene::new();

    // A Scene is just a recorded list of drawing commands: nothing is
    // rasterized until it is handed to a Renderer with a wgpu device.
    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        Color::from_rgb8(240, 140, 160),
        None,
        &RoundedRect::new(20.0, 20.0, 180.0, 120.0, 12.0),
    );
    scene.fill(
        Fill::NonZero,
        Affine::translate((100.0, 140.0)),
        Color::from_rgb8(60, 120, 220),
        None,
        &Circle::new((0.0, 0.0), 40.0),
    );
    println!("scene encoded");
}
