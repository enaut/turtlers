//! Test circle_left and circle_right commands

use macroquad::prelude::*;
use turtle_lib_macroquad::*;

#[macroquad::main("Circle Test")]
async fn main() {
    // Create a turtle plan
    let mut plan = create_turtle();

    plan.right(45.0);
    plan.forward(100.0);
    plan.right(45.0);
    plan.forward(100.0);
    //plan.circle_left(100.0, 90.0, 72); // Full circle to the left

    // Create turtle app with animation (speed = 100 pixels/sec)
    let mut app = TurtleApp::new().with_commands(plan.build(), 10.0);

    // Main loop
    loop {
        clear_background(WHITE);

        // Update and render
        app.update();
        app.render();

        next_frame().await
    }
}
