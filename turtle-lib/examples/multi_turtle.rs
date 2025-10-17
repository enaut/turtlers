//! Multi-turtle example demonstrating multiple independent turtles drawing simultaneously
//!
//! This example shows how to:
//! - Create multiple turtle instances using `add_turtle()`
//! - Control each turtle independently with separate command queues
//! - Position turtles at different locations using `go_to()`
//! - Use different colors and pen widths for each turtle
//! - Combine all turtle animations in a single rendering loop

use macroquad::{miniquad::window::set_window_size, prelude::*};
use turtle_lib::*;

#[macroquad::main("Multi-Turtle Example")]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // Default to showing info-level logs if RUST_LOG is not set
                tracing_subscriber::EnvFilter::new("turtle_lib=error")
            }),
        )
        .with_target(true) // Show which module the log came from
        .with_thread_ids(false)
        .with_line_number(true) // Show line numbers
        .with_file(false)
        .without_time()
        .init();

    let mut app = TurtleApp::new();
    set_window_size(1900, 1000);

    // Turtle 0 (default turtle) - Draw a spiral (red)
    let mut turtle0 = create_turtle();
    turtle0.right(45.0);
    turtle0.set_speed(1900.0);
    turtle0.set_pen_color(RED);
    turtle0.set_fill_color(RED);
    turtle0.set_pen_width(2.0);
    turtle0.begin_fill();
    for i in 0..36 {
        turtle0.forward(5.0 + i as f32 * 2.0).right(10.0);
    }
    turtle0.pen_up();
    turtle0.go_to(vec2(-200.0, -200.0));
    turtle0.pen_down();
    turtle0.circle_left(30.0, 360.0, 36);
    turtle0.end_fill();

    // Turtle 1 - Draw a square (blue)
    let turtle1_id = app.add_turtle();
    let mut turtle1 = create_turtle();
    turtle1.set_speed(1900.0);
    turtle1.pen_up();
    turtle1.go_to(vec2(-200.0, 0.0));
    turtle1.pen_down();
    turtle1.set_pen_color(BLUE);
    turtle1.set_fill_color(BLUE);
    turtle1.set_pen_width(3.0);
    turtle1.begin_fill();
    for _ in 0..4 {
        turtle1.forward(100.0).right(90.0);
    }
    turtle1.end_fill();

    // Turtle 2 - Draw a hexagon (green)
    let turtle2_id = app.add_turtle();
    let mut turtle2 = create_turtle();
    turtle2.set_speed(150.0);
    turtle2.pen_up();
    turtle2.go_to(vec2(200.0, 0.0));
    turtle2.pen_down();
    turtle2.set_pen_color(GREEN);
    turtle2.set_pen_width(3.0);
    for _ in 0..6 {
        turtle2.forward(80.0).right(60.0);
    }

    // Turtle 3 - Draw a star (yellow)
    let turtle3_id = app.add_turtle();
    let mut turtle3 = create_turtle();
    turtle3.set_fill_color(ORANGE);
    turtle3.begin_fill();
    turtle3.set_speed(150.0);
    turtle3.pen_up();
    turtle3.go_to(vec2(0.0, 150.0));
    turtle3.pen_down();
    turtle3.set_pen_color(YELLOW);
    turtle3.set_pen_width(3.0);
    for _ in 0..5 {
        turtle3.forward(120.0).right(144.0);
    }
    turtle3.end_fill();
    // Turtle 4 - Draw a filled circle (purple)
    let turtle4_id = app.add_turtle();
    let mut turtle4 = create_turtle();
    turtle4.set_speed(150.0);
    turtle4.pen_up();
    turtle4.go_to(vec2(0.0, -150.0));
    turtle4.pen_down();
    turtle4.set_pen_color(PURPLE);
    turtle4.set_fill_color(Color::new(0.5, 0.0, 0.5, 0.5));
    turtle4.begin_fill();
    turtle4.circle_left(60.0, 360.0, 36);
    turtle4.end_fill();

    // Add all commands to the app
    app = app.with_commands(turtle0.build());
    app = app.with_commands_for_turtle(turtle1_id, turtle1.build());
    app = app.with_commands_for_turtle(turtle2_id, turtle2.build());
    app = app.with_commands_for_turtle(turtle3_id, turtle3.build());
    app = app.with_commands_for_turtle(turtle4_id, turtle4.build());

    // Main loop
    loop {
        clear_background(WHITE);
        app.update();
        app.render();

        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
            break;
        }

        next_frame().await;
    }
}
