use turtle_lib::*;

#[turtle_main("Squares")]
fn draw(turtle: &mut TurtlePlan) {
    turtle.set_speed(1000);
    for i in 0..36 {
        let base_color = if i % 2 == 0 {
            Color::new(1.0, 0.0, 0.0, 1.0) // red
        } else {
            Color::new(1.0, 1.0, 1.0, 1.0) // white
        };
        let alpha = (1.0 - i as f64 / 54.0) as f32;
        let fill_color = Color::new(base_color.r, base_color.g, base_color.b, alpha);
        turtle.set_fill_color(fill_color);
        turtle.begin_fill();
        square(turtle);
        turtle.end_fill();
        turtle.right(10.0);
    }
}

fn square(turtle: &mut TurtlePlan) {
    for _ in 0..4 {
        turtle.forward(200.0);
        turtle.right(90.0);
    }
}
