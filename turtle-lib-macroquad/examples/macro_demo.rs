//! Simple demo of the turtle_main macro
//!
//! This example shows how the turtle_main macro simplifies turtle programs
//! by automatically handling window setup, turtle creation, and the render loop.

use turtle_lib_macroquad::*;

#[turtle_main("Macro Demo - Simple Square")]
fn draw_square(turtle: &mut TurtlePlan) {
    turtle.set_pen_color(BLUE);
    turtle.set_pen_width(3.0);

    for _ in 0..4 {
        turtle.forward(150.0);
        turtle.right(90.0);
    }
}
