//! Simple square example demonstrating basic turtle graphics

use macroquad::prelude::*;
use turtle_lib_macroquad::*;

#[macroquad::main("Turtle Square")]
async fn main() {
    // Create a turtle plan
    let mut plan = create_turtle();
    plan.shape(ShapeType::Turtle);
    plan.set_speed(800);

    // Draw a square
    for _ in 0..5 {
        plan.forward(200.0);
        plan.circle_left(10.0, 72.0, 1000);
        plan.circle_right(5.0, 360.0, 1000);
        plan.circle_left(10.0, 72.0, 1000);
    }

    // Create turtle app with animation (speed = 100 pixels/sec)
    let mut app = TurtleApp::new().with_commands(plan.build(), 100.0);

    // Main loop
    loop {
        clear_background(WHITE);

        // Update and render
        app.update();
        app.render();

        next_frame().await
    }
}
