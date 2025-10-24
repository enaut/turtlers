//! Yin-Yang symbol example demonstrating multi-contour fills

use turtle_lib::*;

#[turtle_main("Yin-Yang")]
fn draw(turtle: &mut TurtlePlan) {
    turtle
        .set_speed(100)
        .circle_left(90.0, 180.0, 36)
        .begin_fill()
        .circle_left(90.0, 180.0, 36)
        .circle_left(45.0, 180.0, 26)
        .circle_right(45.0, 180.0, 26)
        .pen_up()
        .right(90.0)
        .forward(37.0)
        .left(90.0)
        .pen_down()
        .circle_right(8.0, 360.0, 12)
        .pen_up()
        .right(90.0)
        .forward(90.0)
        .left(90.0)
        .pen_down()
        .circle_right(8.0, 360.0, 12)
        .end_fill();
}
