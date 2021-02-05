use rand;
use rgb::RGB;
use draw::{Canvas, Drawing, Shape, Style, SvgRenderer};

fn update_pos(min_pos:f32, max_pos:f32, current:f32) ->  f32 {
    
    (current + if rand::random() { 1.0 } else { -1.0 }).max(min_pos).min(max_pos)
}

fn update_color(current : u8) ->  u8{
    let offset = 10;
    (
        if rand::random() { current.checked_add(offset) } else { current.checked_sub(offset)}
    ).unwrap_or(current)
}
fn main() {
    let length : u16 = 100;
    let mut canvas = Canvas::new(u32::from(length), u32::from(length));

    let how_many = 10000;
    let mut x_pos = 10.0;
    let mut y_pos = 10.0;
    let mut r = 128;
    let mut g = 128;
    let mut b = 128;

    let update_pos = move |pos| update_pos(0.0,f32::from(length), pos);  

    for _ in 0..how_many {
        x_pos = update_pos(x_pos);
        y_pos = update_pos(y_pos);
        r = update_color(r);
        g = update_color(g);
        b = update_color(b);
        
        let circle = Drawing::new()
            .with_shape(Shape::Circle { radius: 1 })
            .with_xy(x_pos, y_pos)
            .with_style(Style::filled(RGB::new(r,g,b)));

        canvas.display_list.add(circle);
    }

    draw::render::save(&canvas, "target/tests/svg/output.svg", SvgRenderer::new())
        .expect("Failed to save");
}
