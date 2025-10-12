//! Example matching the original requirements exactly

use turtle_lib_macroquad::*;

#[turtle_main("Fill Example - Original Requirements")]
fn draw(turtle: &mut TurtlePlan) {
    turtle.right(90.0);
    turtle.set_pen_width(3.0);
    turtle.set_speed(900);

    turtle.set_pen_color(BLUE);
    turtle.set_fill_color(RED);
    turtle.begin_fill();

    turtle.circle_left(100.0, 360.0, 16);

    // Draw a circle (36 small steps)
    for _ in 0..36 {
        turtle.forward(5.0);
        turtle.right(10.0);
    }

    turtle.end_fill();

    // Draw a square with no fill
    turtle.set_pen_color(GREEN);
    turtle.forward(120.0);
    for _ in 0..3 {
        turtle.right(90.0);
        turtle.forward(240.0);
    }
    turtle.right(90.0);
    turtle.forward(120.0);

    // Set speed for animation
    turtle.set_speed(200);
}
