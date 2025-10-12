//! Celebrates the 1.0.0 release of the original sunjay/turtle library.
//!
//! This example draws "1.0.0" with decorative background lines and filled shapes.
//! Ported from the original sunjay/turtle example.

use turtle_lib::*;

#[turtle_main("Version 1.0.0")]
fn draw_version(turtle: &mut TurtlePlan) {
    turtle.set_pen_width(10.0);
    turtle.set_speed(999); // instant
    turtle.pen_up();
    turtle.go_to(vec2(350.0, 178.0));
    turtle.pen_down();

    bg_lines(turtle);

    turtle.pen_up();
    turtle.go_to(vec2(-270.0, -200.0));
    turtle.set_heading(90.0);
    turtle.pen_down();

    turtle.set_speed(100); // normal
    turtle.set_pen_color(BLUE);
    // Cyan with alpha - using RGB values for Color::from("#00E5FF")
    turtle.set_fill_color([0.0, 0.898, 1.0, 0.75]);

    one(turtle);

    turtle.set_speed(200); // faster

    turtle.pen_up();
    turtle.left(90.0);
    turtle.backward(50.0);
    turtle.pen_down();

    small_circle(turtle);

    turtle.pen_up();
    turtle.backward(150.0);
    turtle.pen_down();

    zero(turtle);

    turtle.pen_up();
    turtle.backward(150.0);
    turtle.pen_down();

    small_circle(turtle);

    turtle.pen_up();
    turtle.backward(150.0);
    turtle.pen_down();

    zero(turtle);
}

fn bg_lines(turtle: &mut TurtlePlan) {
    // Light green color for background lines (#76FF03)
    turtle.set_pen_color([0.463, 1.0, 0.012, 1.0].into());
    turtle.set_heading(165.0);
    turtle.forward(280.0);

    turtle.left(147.0);
    turtle.forward(347.0);

    turtle.right(158.0);
    turtle.forward(547.0);

    turtle.left(138.0);
    turtle.forward(539.0);

    turtle.right(168.0);
    turtle.forward(477.0);

    turtle.left(154.0);
    turtle.forward(377.0);

    turtle.right(158.0);
    turtle.forward(329.0);
}

fn small_circle(turtle: &mut TurtlePlan) {
    turtle.begin_fill();
    for _ in 0..90 {
        turtle.forward(1.0);
        turtle.right(4.0);
    }
    turtle.end_fill();
}

fn one(turtle: &mut TurtlePlan) {
    turtle.begin_fill();
    for _ in 0..2 {
        turtle.forward(420.0);
        turtle.left(90.0);
        turtle.forward(50.0);
        turtle.left(90.0);
    }
    turtle.end_fill();
}

fn zero(turtle: &mut TurtlePlan) {
    turtle.begin_fill();
    for _ in 0..2 {
        arc_right(turtle);
        arc_forward(turtle);
    }
    turtle.end_fill();
}

fn arc_right(turtle: &mut TurtlePlan) {
    // Draw an arc that moves right faster than it moves forward
    for i in 0..90 {
        turtle.forward(3.0);
        turtle.right((90.0 - i as f32) / 45.0);
    }
}

fn arc_forward(turtle: &mut TurtlePlan) {
    // Draw an arc that moves forward faster than it moves right
    for i in 0..90 {
        turtle.forward(3.0);
        turtle.right(i as f32 / 45.0);
    }
}
