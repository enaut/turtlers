//! Simple square example demonstrating basic turtle graphics

use turtle_lib_macroquad::*;

#[turtle_main("Turtle Square")]
fn draw(turtle: &mut TurtlePlan) {
    turtle.shape(ShapeType::Turtle);

    // Draw a square
    for _ in 0..4 {
        turtle.forward(100.0).right(90.0);
    }

    // Set animation speed
    turtle.set_speed(50);
}
