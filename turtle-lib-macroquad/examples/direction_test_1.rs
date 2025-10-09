//! Test circle_left and circle_right commands

use macroquad::prelude::*;
use turtle_lib_macroquad::*;

#[macroquad::main("Circle Test")]
async fn main() {
    // Create a turtle plan
    let mut plan = create_turtle();

    // Set animation speed
    plan.set_speed(50);
    plan.right(45.0);
    plan.forward(100.0);
    plan.right(45.0);
    plan.forward(100.0);

    // Create turtle app with animation (speed = 100 pixels/sec)
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
