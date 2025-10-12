//! Demo of the turtle_main macro with inline code
//!
//! This example shows that you can write your turtle code directly
//! in the function body without taking a turtle parameter.

use turtle_lib::*;

#[turtle_main("Macro Demo - Inline Spiral")]
fn draw_spiral() {
    turtle.set_pen_color(RED);
    turtle.set_pen_width(2.0);

    for i in 0..36 {
        turtle.forward(i as f32 * 3.0);
        turtle.right(25.0);
    }
}
