//! Koch snowflake fractal example

use turtle_lib_macroquad::*;

fn koch(depth: u32, turtle: &mut TurtlePlan, distance: f32) {
    if depth == 0 {
        turtle.forward(distance);
    } else {
        let new_distance = distance / 3.0;
        koch(depth - 1, turtle, new_distance);
        turtle.left(60.0);
        koch(depth - 1, turtle, new_distance);
        turtle.right(120.0);
        koch(depth - 1, turtle, new_distance);
        turtle.left(60.0);
        koch(depth - 1, turtle, new_distance);
    }
}

#[turtle_main("Koch Snowflake")]
fn draw(turtle: &mut TurtlePlan) {
    // Position turtle
    turtle.set_speed(1001);
    turtle.pen_up();
    turtle.backward(150.0);

    turtle.pen_down();

    // Draw Koch snowflake (triangle of Koch curves)
    for _ in 0..3 {
        koch(4, turtle, 300.0);
        turtle.right(120.0);
        turtle.set_speed(1200);
    }

    turtle.hide(); // Hide turtle when done
}
