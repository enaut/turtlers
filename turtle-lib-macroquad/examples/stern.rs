//! Star pattern example demonstrating complex turtle patterns

use turtle_lib_macroquad::*;

#[turtle_main("Star Pattern")]
fn draw(turtle: &mut TurtlePlan) {
    turtle.shape(ShapeType::Turtle);
    turtle.set_speed(1500);
    turtle.set_pen_width(0.5);

    // Draw a 5-pointed star pattern repeatedly
    for _i in 0..50000 {
        turtle.forward(200.0);
        turtle.circle_left(10.0, 72.0, 1000);
        turtle.circle_right(5.0, 360.0, 1000);
        turtle.circle_left(10.0, 72.0, 1000);
    }

    // Set animation speed
    turtle.set_speed(300);
}
