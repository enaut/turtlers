//! Simple square example demonstrating basic turtle graphics

use macroquad::prelude::*;
use turtle_lib_macroquad::*;

#[macroquad::main("Turtle Square")]
async fn main() {
    // Create a turtle plan
    let mut t = create_turtle();

    t.circle_left(90.0, 180.0, 36);
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

    // Set animation speed
    t.set_speed(1000);

    // Create turtle app with animation (speed = 100 pixels/sec)
    let mut app = TurtleApp::new().with_commands(t.build());

    // Main loop
    loop {
        clear_background(WHITE);

        // Update and render
        app.update();
        app.render();

        next_frame().await
    }
}
