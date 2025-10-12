//! Minimal turtle example - just 10 lines!
//!
//! This is the simplest possible turtle program using the macro.

use turtle_lib_macroquad::*;

#[turtle_main("Hello Turtle")]
fn hello() {
    turtle.set_pen_color(BLUE);
    for _ in 0..4 {
        turtle.forward(100.0);
        turtle.right(90.0);
    }
}
