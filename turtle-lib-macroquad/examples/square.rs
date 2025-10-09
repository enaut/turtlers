//! Simple square example demonstrating basic turtle graphics

use macroquad::prelude::*;
use turtle_lib_macroquad::*;

#[macroquad::main("Turtle Square")]
async fn main() {
    // Create a turtle plan
    let mut plan = create_turtle();
    plan.shape(ShapeType::Turtle);

    // Draw a square
    for _ in 0..4 {
        plan.forward(100.0).right(90.0);
    }

    // Set animation speed
    plan.set_speed(50);

    // Create turtle app with animation
    let mut app = TurtleApp::new().with_commands(plan.build());

    // Main loop
    loop {
        clear_background(WHITE);

        // Update and render
        app.update();
        app.render();

        next_frame().await
    }
}
