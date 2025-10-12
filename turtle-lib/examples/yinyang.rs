//! Yin-Yang symbol example demonstrating multi-contour fills

use turtle_lib::*;

#[turtle_main("Yin-Yang")]
fn draw(turtle: &mut TurtlePlan) {
    turtle.set_speed(900);

    turtle.circle_left(90.0, 180.0, 36);
    turtle.begin_fill();
    turtle.circle_left(90.0, 180.0, 36);
    turtle.circle_left(45.0, 180.0, 26);
    turtle.circle_right(45.0, 180.0, 26);
    turtle.pen_up();
    turtle.right(90.0);
    turtle.forward(37.0);
    turtle.left(90.0);
    turtle.pen_down();
    turtle.circle_right(8.0, 360.0, 12);
    turtle.pen_up();
    turtle.right(90.0);
    turtle.forward(90.0);
    turtle.left(90.0);
    turtle.pen_down();
    turtle.circle_right(8.0, 360.0, 12);
    turtle.end_fill();

    // Set animation speed
    turtle.set_speed(1000);
}
