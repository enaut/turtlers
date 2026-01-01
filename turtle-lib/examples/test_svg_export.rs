//! Test example for CLI SVG export feature
//!
//! Run this with: cargo run --package turtle-lib --example test_svg_export --features svg -- --export-svg test_output.svg

use turtle_lib::*;

#[turtle_main("SVG Export Test")]
fn draw_test(turtle: &mut TurtlePlan) {
    turtle.set_pen_color(RED);
    turtle.set_pen_width(3.0);
    
    // Draw a square
    for _ in 0..4 {
        turtle.forward(100.0);
        turtle.right(90.0);
    }
    
    // Draw a circle
    turtle.set_pen_color(BLUE);
    turtle.pen_up();
    turtle.forward(150.0);
    turtle.pen_down();
    turtle.circle_left(50.0, 360.0, 36);
    
    // Draw a filled triangle
    turtle.set_fill_color(GREEN);
    turtle.pen_up();
    turtle.go_to(vec2(-50.0, 100.0));
    turtle.pen_down();
    turtle.begin_fill();
    for _ in 0..3 {
        turtle.forward(80.0);
        turtle.right(120.0);
    }
    turtle.end_fill();
}
