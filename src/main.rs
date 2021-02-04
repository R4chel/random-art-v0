use draw::{Canvas, Color, Drawing, Shape, Style, SvgRenderer};
 
fn main() {
    
    let mut canvas = Canvas::new(100, 100);

    let circle = Drawing::new()
        .with_shape(Shape::Circle {radius : 5 })
        .with_xy(10.0, 10.0)
        .with_style(Style::filled(Color::random()));

    canvas.display_list.add(circle);

    draw::render::save(
        &canvas,
        "target/tests/svg/output.svg",
        SvgRenderer::new(),
    )
        .expect("Failed to save");
}
