//! Cheese example - demonstrates multi-contour fills with holes
//!
//! This example creates a cheese-like shape by:
//! 1. Drawing the outer square boundary
//! 2. Lifting the pen (pen_up) to close that contour
//! 3. Drawing circular and triangular holes with pen_up/pen_down
//!
//! Lyon's EvenOdd fill rule automatically creates holes where contours overlap!

use macroquad::prelude::*;
use turtle_lib::*;

#[macroquad::main("Cheese with Holes")]
async fn main() {
    let mut turtle = create_turtle();

    // Set fill color to yellow (cheese color!)
    turtle.set_fill_color(YELLOW);
    turtle.set_pen_color(ORANGE);
    turtle.set_pen_width(3.0);

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
    // Execute the plan
    let mut app = TurtleApp::new().with_commands(turtle.build());

    loop {
        clear_background(Color::new(0.95, 0.95, 0.98, 1.0));
        app.update();
        app.render();

        // Instructions
        draw_text(
            "Cheese with Holes - pen_up/pen_down creates multiple contours!",
            10.0,
            20.0,
            18.0,
            BLACK,
        );
        draw_text("Press ESC or Q to quit", 10.0, 40.0, 16.0, DARKGRAY);

        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
            break;
        }

        next_frame().await;
    }
}
