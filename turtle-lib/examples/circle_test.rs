//! Test circle_left and circle_right commands

use turtle_lib::*;

#[turtle_main("Circle Test")]
fn draw(turtle: &mut TurtlePlan) {
    turtle.shape(ShapeType::Turtle);

    // Draw some circles
    turtle.set_pen_color(RED);
    turtle.set_pen_width(0.5);
    turtle.left(90.0);
    turtle.set_speed(999);
    turtle.circle_left(100.0, 540.0, 72); // partial circle to the left

    turtle.begin_fill();
    turtle.forward(150.0);
    turtle.set_speed(100);
    turtle.set_pen_color(BLUE);
    turtle.circle_right(50.0, 270.0, 72); // partial circle to the right
                                          // Set animation speed
    turtle.end_fill();
    turtle.set_speed(20);
    turtle.forward(150.0);
    turtle.circle_left(50.0, 180.0, 12);
    turtle.circle_right(50.0, 180.0, 12);

    turtle.set_speed(700);
    turtle.set_pen_color(GREEN);
    turtle.circle_left(50.0, 180.0, 36); // Half circle to the left
}
