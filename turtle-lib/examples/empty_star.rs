//! Draw an empty 5-pointed star using straight lines.
//! Ported from the turtle crate example.

use turtle_lib::*;

#[turtle_main("Empty Star")]
fn draw(turtle: &mut TurtlePlan) {
    let points = 5.0;
    let angle = 180.0 / points;

    turtle
        .set_pen_width(4.0)
        .set_pen_color(YELLOW)
        .pen_up()
        .forward(150.0)
        .right(180.0 - angle / 2.0)
        .pen_down();

    for _ in 0..5 {
        turtle
            .forward(100.0)
            .left(angle * 2.0)
            .forward(100.0)
            .right(180.0 - angle);
    }
}
