use draw::{Canvas, Drawing, Shape, Style, SvgRenderer};
use rand::prelude::*;
use rgb::RGB;

fn update_pos(min_pos: f32, max_pos: f32, current: f32) -> f32 {
    let offset = 1.0;
    (current
        + if rand::random() {
            1.0 * offset
        } else {
            -1.0 * offset
        })
    .max(min_pos)
    .min(max_pos)
}

fn update_color(current: u8) -> u8 {
    let offset = 10;
    (if rand::random() {
        current.checked_add(offset)
    } else {
        current.checked_sub(offset)
    })
    .unwrap_or(current)
}
fn main() {
    let length: u16 = 100;
    let mut canvas = Canvas::new(u32::from(length), u32::from(length));
    let mut rng = thread_rng();

    let max_pos = f32::from(length);
    let how_many = 5000;
    let mut x_pos = 10.0;
    let mut y_pos = 10.0;
    let mut r = rng.gen::<u8>();
    let mut g = rng.gen::<u8>();
    let mut b = rng.gen::<u8>();

    let update_pos = move |pos| update_pos(0.0, max_pos, pos);

    for _ in 0..how_many {
        x_pos = update_pos(x_pos);
        y_pos = update_pos(y_pos);
        r = update_color(r);
        g = update_color(g);
        b = update_color(b);

        let circle = Drawing::new()
            .with_shape(Shape::Circle { radius: 1 })
            .with_xy(x_pos, y_pos)
            .with_style(Style::filled(RGB::new(r, g, b)));

        canvas.display_list.add(circle);
    }

    draw::render::save(&canvas, "target/tests/svg/output.svg", SvgRenderer::new())
        .expect("Failed to save");
}
