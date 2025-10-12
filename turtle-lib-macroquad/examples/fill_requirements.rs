//! Example matching the original requirements exactly

use macroquad::prelude::*;
use turtle_lib_macroquad::*;

#[macroquad::main("Fill Example - Original Requirements")]
async fn main() {
    let mut turtle = create_turtle();

    turtle.right(90.0);
    turtle.set_pen_width(3.0);
    turtle.set_speed(900);

    turtle.set_pen_color(BLUE);
    turtle.set_fill_color(RED);
    turtle.begin_fill();

    turtle.circle_left(100.0, 360.0, 16);

    // Draw a circle (36 small steps)
    for _ in 0..36 {
        turtle.forward(5.0);
        turtle.right(10.0);
    }

    turtle.end_fill();

    // Draw a square with no fill
    turtle.set_pen_color(GREEN);
    turtle.forward(120.0);
    for _ in 0..3 {
        turtle.right(90.0);
        turtle.forward(240.0);
    }
    turtle.right(90.0);
    turtle.forward(120.0);

    // Set speed for animation
    turtle.set_speed(200);

    let mut app = TurtleApp::new().with_commands(turtle.build());

    loop {
        clear_background(WHITE);
        app.update();
        app.render();

        // Instructions
        draw_text(
            "Fill Example - Circle filled with red, square not filled",
            10.0,
            20.0,
            20.0,
            BLACK,
        );
        draw_text(
            "Mouse: drag to pan, scroll to zoom",
            10.0,
            40.0,
            16.0,
            DARKGRAY,
        );

        next_frame().await
    }
}
