//! Example demonstrating different turtle shapes

use macroquad::prelude::*;
use turtle_lib_macroquad::*;

#[macroquad::main("Turtle Shapes")]
async fn main() {
    // Create a turtle plan that demonstrates different shapes
    let mut plan = create_turtle();

    // Start with triangle (default)
    plan.forward(100.0);
    plan.right(90.0);

    // Change to turtle shape
    plan.shape(ShapeType::Turtle);
    plan.forward(100.0);
    plan.right(90.0);

    // Change to circle
    plan.shape(ShapeType::Circle);
    plan.forward(100.0);
    plan.right(90.0);

    // Change to square
    plan.shape(ShapeType::Square);
    plan.forward(100.0);
    plan.right(90.0);

    // Change to arrow
    plan.shape(ShapeType::Arrow);
    plan.forward(100.0);

    // Set animation speed
    plan.set_speed(50);

    // Create turtle app with animation (speed = 100 pixels/sec for slower animation)
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
