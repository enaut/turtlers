//! Advanced fill example with multiple holes and complex shapes
//!
//! This example uses manual setup to demonstrate custom window size and UI elements.

use macroquad::{miniquad::window::set_window_size, prelude::*};
use turtle_lib::*;

#[macroquad::main("Advanced Fill Demo")]
async fn main() {
    set_window_size(2000, 1900);
    let mut t = create_turtle_plan();

    // Example 1: Star shape (concave polygon)
    t.pen_up();
    t.go_to(vec2(-200.0, 100.0));
    t.pen_down();
    t.set_heading(0.0);

    t.set_fill_color(GOLD);
    t.set_pen_color(ORANGE);
    t.set_pen_width(2.0);
    t.set_speed(500);

    t.begin_fill();
    // Draw 5-pointed star
    for _ in 0..5 {
        t.forward(100.0);
        t.right(144.0);
    }
    t.end_fill();

    // Example 2: Swiss cheese (polygon with multiple holes)
    t.pen_up();
    t.go_to(vec2(100.0, 100.0));
    t.pen_down();
    t.set_heading(0.0);

    t.set_fill_color(YELLOW);
    t.set_pen_color(ORANGE);

    t.begin_fill();

    // Outer square
    for _ in 0..4 {
        t.forward(150.0);
        t.right(90.0);
    }

    // First hole (circle)
    t.pen_up();
    t.go_to(vec2(140.0, 130.0));
    t.pen_down();
    t.circle_right(150.0, 360.0, 36);

    // Second hole (circle)
    t.pen_up();
    t.go_to(vec2(200.0, 170.0));
    t.pen_down();
    t.circle_right(10.0, 360.0, 36);

    // Third hole (triangle)
    t.pen_up();
    t.go_to(vec2(160.0, 200.0));
    t.pen_down();
    t.circle_right(15.0, 360.0, 3);

    // Fourth hole (square)
    t.pen_up();
    t.go_to(vec2(190.0, 200.0));
    t.pen_down();
    t.circle_right(15.0, 360.0, 4);

    // fifth hole (pentagon)
    t.pen_up();
    t.go_to(vec2(230.0, 200.0));
    t.pen_down();
    t.circle_right(15.0, 360.0, 5);

    t.end_fill();

    // Example 3: Donut (circle with circular hole)
    t.pen_up();
    t.go_to(vec2(-100.0, -100.0));
    t.pen_down();
    t.set_heading(0.0);

    t.set_fill_color(Color::new(0.8, 0.4, 0.2, 1.0));
    t.set_pen_color(Color::new(0.6, 0.3, 0.1, 1.0));

    t.begin_fill();

    // Outer circle
    for _ in 0..72 {
        t.forward(3.0);
        t.right(5.0);
    }

    // Move to inner circle
    t.pen_up();
    t.go_to(vec2(-75.0, -90.0));
    t.pen_down();

    // Inner circle (hole)
    for _ in 0..72 {
        t.forward(1.5);
        t.right(5.0);
    }

    t.end_fill();

    // Set animation speed
    t.set_speed(500);

    let mut app = TurtleApp::new().with_commands(t.build());

    let target_fps = 1.0; // 1 frame per second for debugging
    let frame_time = 1.0 / target_fps;
    let mut last_frame_time = macroquad::time::get_time();

    loop {
        // Frame rate limiting
        let current_time = macroquad::time::get_time();
        let delta = current_time - last_frame_time;

        if delta < frame_time {
            //  std::thread::sleep(std::time::Duration::from_secs_f64(frame_time - delta));
        }
        last_frame_time = macroquad::time::get_time();

        clear_background(Color::new(0.95, 0.95, 0.98, 1.0));
        app.update();
        app.render();

        // Instructions
        draw_text(
            "Advanced Fill Demo: Star, Swiss Cheese, Donut",
            10.0,
            20.0,
            20.0,
            BLACK,
        );
        draw_text(
            "Features: concave polygons, multiple holes, pen_up during fill",
            10.0,
            40.0,
            16.0,
            DARKGRAY,
        );
        draw_text(
            "Mouse: drag to pan, scroll to zoom",
            10.0,
            60.0,
            16.0,
            DARKGRAY,
        );

        next_frame().await
    }
}
