//! Fill demonstration with holes

use macroquad::prelude::*;
use turtle_lib_macroquad::*;

#[macroquad::main("Fill Demo")]
async fn main() {
    let mut t = create_turtle();

    // Example from requirements: circle with hole (like a donut)
    t.set_pen_color(BLUE);
    t.set_pen_width(3.0);
    t.right(90.0);

    // Set fill color and begin fill
    t.set_fill_color(RED);
    t.begin_fill();

    // Outer circle
    t.circle_right(150.0, 360.0, 72);

    // Move to start of inner circle (hole)
    // pen_up doesn't matter for fill - vertices still recorded!
    t.pen_up();
    t.forward(50.0);
    t.pen_down();

    // Inner circle (creates a hole)
    t.circle_right(150.0, 360.0, 72);

    t.end_fill();

    // Draw a square with no fill
    t.pen_up();
    t.forward(100.0);
    t.pen_down();
    t.set_pen_color(GREEN);

    for _ in 0..4 {
        t.forward(100.0);
        t.right(90.0);
    }

    // Set animation speed
    t.set_speed(100);

    let mut app = TurtleApp::new().with_commands(t.build());

    loop {
        clear_background(WHITE);
        app.update();
        app.render();
        next_frame().await
    }
}
