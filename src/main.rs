use draw::{Canvas, Color, Drawing, Shape, Style, SvgRenderer};
use rand::prelude::*;

fn main() {
    let mut canvas = Canvas::new(100, 100);

    let how_many: u8 = 3;
    let mut x_pos = 10.0;
    let mut y_pos = 10.0;

    let mut rng = thread_rng();

    for _ in 0..how_many {
        x_pos = &x_pos + if rng.gen() { 1.0 } else { -1.0 };

        y_pos = &y_pos + if rng.gen() { 1.0 } else { -1.0 };

        let circle = Drawing::new()
            .with_shape(Shape::Circle { radius: 1 })
            .with_xy(x_pos, y_pos)
            .with_style(Style::filled(Color::random()));

        canvas.display_list.add(circle);
    }

    draw::render::save(&canvas, "target/tests/svg/output.svg", SvgRenderer::new())
        .expect("Failed to save");
}
