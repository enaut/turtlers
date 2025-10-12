//! Cheese example using the turtle_main macro
//!
//! This is a simplified version of cheese.rs that demonstrates how the
//! turtle_main macro reduces boilerplate code.

use turtle_lib::*;

#[turtle_main("Cheese with Holes - Using Macro")]
fn draw_cheese(turtle: &mut TurtlePlan) {
    // Set fill color to yellow (cheese color!)
    turtle.set_pen_color(ORANGE);
    turtle.set_pen_width(3.0);
    turtle.set_fill_color(YELLOW);

    println!("=== Starting cheese fill ===");
    turtle.begin_fill();

    // Draw outer boundary (large square)
    println!("Drawing outer square boundary...");
    for _ in 0..4 {
        turtle.forward(400.0);
        turtle.right(90.0);
    }

    // Close outer contour and start drawing holes
    println!("Closing outer contour with pen_up");
    turtle.pen_up();

    // Draw triangular hole in the middle
    println!("Drawing triangular hole...");
    turtle.go_to(vec2(200.0, 120.0));
    turtle.pen_down(); // Start new contour for hole

    for _ in 0..3 {
        turtle.forward(160.0);
        turtle.right(120.0);
    }

    println!("Closing triangle contour with pen_up");
    turtle.pen_up(); // Close triangle hole contour

    // Draw circular hole (top-left) using circle_left
    println!("Drawing circular hole (top-left) with circle_left...");
    turtle.go_to(vec2(100.0, 100.0));
    turtle.pen_down(); // Start new contour for hole
    turtle.circle_left(30.0, 360.0, 36); // radius=30, full circle, 36 steps
    println!("Closing circle contour with pen_up");
    turtle.pen_up(); // Close circle hole contour

    // Draw circular hole (bottom-right) using circle_right
    println!("Drawing circular hole (bottom-right) with circle_right...");
    turtle.go_to(vec2(280.0, 280.0));
    turtle.pen_down(); // Start new contour for hole
    turtle.circle_right(40.0, 360.0, 36); // radius=40, full circle, 36 steps
    println!("Closing circle contour with pen_up");
    turtle.pen_up(); // Close circle hole contour

    // End fill - Lyon will automatically create holes!
    println!("Calling end_fill - Lyon should create holes now!");
    turtle.end_fill();

    // Set animation speed
    turtle.set_speed(300);

    println!("Building and executing turtle plan...");
}
