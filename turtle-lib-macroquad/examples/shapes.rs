//! Example demonstrating different turtle shapes

use turtle_lib_macroquad::*;

#[turtle_main("Turtle Shapes")]
fn draw(turtle: &mut TurtlePlan) {
    // Start with triangle (default)
    turtle.forward(100.0);
    turtle.right(90.0);

    // Change to turtle shape
    turtle.shape(ShapeType::Turtle);
    turtle.forward(100.0);
    turtle.right(90.0);

    // Change to circle
    turtle.shape(ShapeType::Circle);
    turtle.forward(100.0);
    turtle.right(90.0);

    // Change to square
    turtle.shape(ShapeType::Square);
    turtle.forward(100.0);
    turtle.right(90.0);

    // Change to arrow
    turtle.shape(ShapeType::Arrow);
    turtle.forward(100.0);

    // Set animation speed
    turtle.set_speed(50);
}
