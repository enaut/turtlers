//! Test circle_left and circle_right commands

use macroquad::prelude::*;
use turtle_lib_macroquad::*;

#[macroquad::main("Circle Test")]
async fn main() {
    // Create a turtle plan
    let mut plan = create_turtle();
    plan.shape(ShapeType::Turtle);

    // Draw some circles
    plan.set_pen_color(RED);
    plan.set_pen_width(0.5);
    plan.left(90.0);
    plan.set_speed(999);
    plan.circle_left(100.0, 540.0, 72); // partial circle to the left

    plan.forward(150.0);
    plan.set_speed(100);
    plan.set_pen_color(BLUE);
    plan.circle_right(50.0, 270.0, 72); // partial circle to the right
                                        // Set animation speed
    plan.set_speed(20);
    plan.forward(150.0);
    plan.circle_left(50.0, 180.0, 12);
    plan.circle_right(50.0, 180.0, 12);

    plan.set_speed(700);
    plan.set_pen_color(GREEN);
    plan.circle_left(50.0, 180.0, 36); // Half circle to the left

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
