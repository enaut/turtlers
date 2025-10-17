//! Example demonstrating how to enable logging/tracing output from the turtle library
//!
//! This example shows how to use `tracing-subscriber` to see debug output from the library.
//! You can control the log level using the `RUST_LOG` environment variable:
//!
//! ```bash
//! # Show all debug output from turtle-lib
//! RUST_LOG=turtle_lib=debug cargo run --example logging_example
//!
//! # Show only warnings and errors
//! RUST_LOG=turtle_lib=warn cargo run --example logging_example
//!
//! # Show trace-level output (very verbose, includes all vertices)
//! RUST_LOG=turtle_lib=trace cargo run --example logging_example
//!
//! # Show debug output from specific modules
//! RUST_LOG=turtle_lib::tessellation=debug cargo run --example logging_example
//! RUST_LOG=turtle_lib::execution=debug cargo run --example logging_example
//! ```
//!
//! Note: This example uses manual setup to demonstrate custom initialization logic.

use macroquad::prelude::*;
use turtle_lib::*;

#[macroquad::main("Turtle Logging Example")]
async fn main() {
    // Initialize tracing subscriber to see debug output
    // This will respect the RUST_LOG environment variable
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // Default to showing info-level logs if RUST_LOG is not set
                tracing_subscriber::EnvFilter::new("turtle_lib=trace")
            }),
        )
        .with_target(true) // Show which module the log came from
        .with_thread_ids(false)
        .with_line_number(true) // Show line numbers
        .with_file(false)
        .init();

    tracing::info!("Starting turtle graphics example with logging enabled");
    tracing::info!(
        "Try running with: RUST_LOG=turtle_lib=debug cargo run --example logging_example"
    );

    // Create a turtle plan with fill operations to see detailed logging
    let mut t = create_turtle_plan();
    t.set_speed(900);

    // Draw a yin-yang symbol with fills (generates lots of debug output)
    t.circle_left(90.0, 180.0, 36);
    t.begin_fill();
    t.circle_left(90.0, 180.0, 36);
    t.circle_left(45.0, 180.0, 26);
    t.circle_right(45.0, 180.0, 26);
    t.pen_up();
    t.right(90.0);
    t.forward(37.0);
    t.left(90.0);
    t.pen_down();
    t.circle_right(8.0, 360.0, 12);
    t.pen_up();
    t.right(90.0);
    t.forward(90.0);
    t.left(90.0);
    t.pen_down();
    t.circle_right(8.0, 360.0, 12);
    t.end_fill();

    tracing::info!("Turtle plan created, starting animation");

    // Set animation speed
    t.set_speed(100); // Slow animation to see the logs in real-time

    // Create turtle app
    let mut app = TurtleApp::new().with_commands(t.build());

    // Main loop
    loop {
        clear_background(WHITE);

        // Update and render - this is where you'll see debug logs
        app.update();
        app.render();

        // Exit when animation is complete
        if app.is_complete() {
            tracing::info!("Animation complete, press any key to exit");
            if is_key_pressed(KeyCode::Space) {
                break;
            }
        }

        next_frame().await
    }

    tracing::info!("Example finished");
}
