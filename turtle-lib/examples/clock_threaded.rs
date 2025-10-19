//! Animated clock example using threading
//!
//! This example demonstrates how to use turtle command channels for animated updates.
//! A separate thread generates the clock drawing commands every second while the main
//! thread handles rendering via the Macroquad game loop.

use chrono::{Local, Timelike};
use macroquad::prelude::{clear_background, is_key_pressed, next_frame, KeyCode, WHITE};
use turtle_lib::{create_turtle_plan, vec2, DirectionalMovement, Turnable, TurtleApp};

#[macroquad::main("Clock (Threaded)")]
async fn main() {
    const HOURS: i32 = 12;
    const MINUTES: f32 = 60.0;
    const SECONDS: f32 = 60.0;
    const FULL_CIRCLE: f32 = 360.0;

    let mut app = TurtleApp::new();
    let turtle_tx = app.create_turtle_channel(10);

    // Spawn a thread that generates clock commands every second
    std::thread::spawn(move || {
        let mut last_second = -1i32;

        loop {
            let now = Local::now();
            let current_second = now.second() as i32;

            // Only generate commands when the time changes
            if current_second != last_second {
                let mut turtle = create_turtle_plan();
                turtle.reset().set_speed(1100).left(90.0);

                // Draw the clock circle and hour markers
                for i in 1..=HOURS {
                    turtle
                        .pen_up()
                        .go_to(vec2(0.0, 0.0))
                        .right(FULL_CIRCLE / HOURS as f32)
                        .forward(205.0);

                    let pen_size = if (i) % 3 == 0 { 7.0 } else { 2.0 };
                    turtle
                        .set_pen_width(pen_size)
                        .pen_down()
                        .forward(10.0)
                        .right(90.0)
                        .pen_up()
                        .backward(pen_size.max(4.0) + (i / 10) as f32 * 4.0)
                        .pen_down()
                        .write_text(format!("{i}"), 2 * pen_size as i32 + 14)
                        .pen_up()
                        .forward(pen_size.max(4.0) + (i / 10) as f32 * 4.0)
                        .pen_down()
                        .left(90.0);
                }

                // Draw the hour hand
                turtle
                    .pen_up()
                    .go_to(vec2(0.0, 0.0))
                    .set_heading(90.0)
                    .right(FULL_CIRCLE / HOURS as f32 * (now.hour() % 12) as f32)
                    .set_pen_width(7.0)
                    .pen_down()
                    .forward(120.0);

                // Draw the minute hand
                turtle
                    .pen_up()
                    .go_to(vec2(0.0, 0.0))
                    .set_heading(90.0)
                    .right(FULL_CIRCLE / MINUTES * now.minute() as f32)
                    .set_pen_width(3.0)
                    .pen_down()
                    .forward(150.0);

                // Draw the second hand
                turtle
                    .pen_up()
                    .go_to(vec2(0.0, 0.0))
                    .set_heading(90.0)
                    .right(FULL_CIRCLE / SECONDS * now.second() as f32)
                    .set_pen_width(1.0)
                    .pen_down()
                    .forward(180.0);

                // Send the command queue to the main thread
                let _ = turtle_tx.send(turtle.build());
                last_second = current_second;
            }

            // Sleep briefly to avoid busy-waiting (update ~10 times per second)
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });

    // Main render loop
    loop {
        clear_background(WHITE);

        // Process any pending commands from the worker thread
        app.process_commands();

        app.update();
        app.render();

        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
            break;
        }

        next_frame().await;
    }
}
