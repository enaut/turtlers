//! Test direction and movement commands

use turtle_lib_macroquad::*;

#[turtle_main("Direction Test")]
fn draw(turtle: &mut TurtlePlan) {
    // Set animation speed
    turtle.set_speed(50);
    turtle.right(45.0);
    turtle.forward(100.0);
    turtle.right(45.0);
    turtle.forward(100.0);
}
