//! Animated clock example
//!
//! This example draws an animated clock that shows the current time.
//! The clock updates every second to reflect the actual time.

use chrono::{Local, Timelike};
use macroquad::prelude::{clear_background, is_key_pressed, next_frame, KeyCode, WHITE};
use turtle_lib::{create_turtle_plan, vec2, DirectionalMovement, Turnable, TurtleApp};

#[macroquad::main("Clock")]
async fn main() {
    const HOURS: i32 = 12;
    const MINUTES: f32 = 60.0;
    const SECONDS: f32 = 60.0;
    const FULL_CIRCLE: f32 = 360.0;

    let mut app = TurtleApp::new();
    let mut last_update = Local::now();

    loop {
        clear_background(WHITE);

        let now = Local::now();

        // Only redraw when the time changes
        if now.second() != last_update.second() {
            let mut turtle = create_turtle_plan();
            turtle.reset().set_speed(1100).left(90.0); // Instant mode for smooth updates

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
                    .write_text(format!("{i}"), 2 * pen_size as i32 + 10)
                    .left(90.0);
            }

            // Draw the hour hand
            turtle
                .pen_up()
                .go_to(vec2(0.0, 0.0))
                .set_heading(90.0)
                .right(FULL_CIRCLE / HOURS as f32 * (now.hour() % 12) as f32)
                .set_pen_width(5.0)
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

            app = TurtleApp::new().with_commands(turtle.build());
            last_update = now;
        }

        app.update();
        app.render();

        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
            break;
        }

        next_frame().await;
    }
}
