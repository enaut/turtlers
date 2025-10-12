//! Fill demonstration with holes

use turtle_lib_macroquad::*;

#[turtle_main("Fill Demo")]
fn draw(turtle: &mut TurtlePlan) {
    // Example from requirements: circle with hole (like a donut)
    turtle.set_pen_color(BLUE);
    turtle.set_pen_width(3.0);
    turtle.right(90.0);

    // Set fill color and begin fill
    turtle.set_fill_color(RED);
    turtle.begin_fill();

    // Outer circle
    turtle.circle_right(150.0, 360.0, 72);

    // Move to start of inner circle (hole)
    // pen_up doesn't matter for fill - vertices still recorded!
    turtle.pen_up();
    turtle.forward(50.0);
    turtle.pen_down();

    // Inner circle (creates a hole)
    turtle.circle_right(150.0, 360.0, 72);

    turtle.end_fill();

    // Draw a square with no fill
    turtle.pen_up();
    turtle.forward(100.0);
    turtle.pen_down();
    turtle.set_pen_color(GREEN);

    for _ in 0..4 {
        turtle.forward(100.0);
        turtle.right(90.0);
    }

    // Set animation speed
    turtle.set_speed(100);
}
