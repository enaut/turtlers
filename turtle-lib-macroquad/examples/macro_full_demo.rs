//! Comprehensive macro example showing various turtle features
//!
//! This example demonstrates:
//! - Colors and pen settings
//! - Fills
//! - Circles
//! - Animation speed

use turtle_lib_macroquad::*;

#[turtle_main("Turtle Macro - Full Demo")]
fn full_demo(turtle: &mut TurtlePlan) {
    // Draw a colorful flower
    turtle.set_speed(200);

    // Center the drawing
    turtle.pen_up();
    turtle.go_to(vec2(200.0, 300.0));
    turtle.pen_down();

    // Draw petals
    for i in 0..8 {
        let hue = i as f32 / 8.0;
        let color = Color::from_rgba(
            ((hue * 360.0).to_radians().sin() * 127.0 + 128.0) as u8,
            ((hue * 360.0 + 120.0).to_radians().sin() * 127.0 + 128.0) as u8,
            ((hue * 360.0 + 240.0).to_radians().sin() * 127.0 + 128.0) as u8,
            255,
        );

        turtle.set_fill_color(color);
        turtle.set_pen_color(color);
        turtle.begin_fill();

        // Draw a petal using circles
        turtle.circle_left(50.0, 180.0, 20);
        turtle.left(90.0);
        turtle.circle_left(50.0, 180.0, 20);
        turtle.left(90.0);

        turtle.end_fill();

        // Move to next petal position
        turtle.right(45.0);
    }

    // Draw center circle
    turtle.pen_up();
    turtle.go_to(vec2(200.0, 300.0));
    turtle.pen_down();
    turtle.set_fill_color(YELLOW);
    turtle.set_pen_color(ORANGE);
    turtle.set_pen_width(2.0);
    turtle.begin_fill();
    turtle.circle_left(20.0, 360.0, 36);
    turtle.end_fill();
}
