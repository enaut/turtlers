//! Demonstration of text rendering with turtle heading orientation

use macroquad::prelude::*;
use turtle_lib::*;

#[turtle_main("Text Demo")]
fn draw(turtle: &mut TurtlePlan) {
    // Write text at heading 0 (right)
    turtle
        .set_pen_color(BLACK)
        .write_text("Heading 0°", 20u16)
        .forward(0.0); // Just to complete the chain

    // Move forward and turn, then write text at different angles
    turtle
        .forward(100.0)
        .right(45.0)
        .write_text("45° right", 18u16)
        .forward(0.0);

    turtle
        .forward(80.0)
        .right(45.0)
        .write_text("90° down", 18u16)
        .forward(0.0);

    turtle
        .forward(80.0)
        .right(45.0)
        .write_text("135°", 18u16)
        .forward(0.0);

    turtle
        .forward(80.0)
        .right(45.0)
        .write_text("180° left", 18u16)
        .forward(0.0);

    // Use different font sizes
    turtle
        .pen_up()
        .go_to(vec2(-200.0, 100.0))
        .pen_down()
        .set_pen_color(BLUE)
        .write_text("Small", 12f32)
        .forward(50.0)
        .write_text("Medium", 20)
        .forward(50.0)
        .write_text("Large", 28u16)
        .forward(0.0);

    // Example with drawing
    turtle
        .pen_up()
        .go_to(vec2(0.0, -150.0))
        .pen_down()
        .set_pen_color(RED)
        .circle_right(50.0, 360.0, 32)
        .pen_up()
        .go_to(vec2(0.0, -150.0))
        .pen_down()
        .write_text("Circle", 16f32)
        .forward(0.0);
}
