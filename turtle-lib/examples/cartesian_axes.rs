//! Cartesian coordinate system example
//!
//! This example draws a cartesian coordinate system with X and Y axes,
//! labeled axis endpoints, and 4 labeled points (one in each quadrant).

use macroquad::prelude::*;
use turtle_lib::*;

#[turtle_main("Cartesian Axes")]
fn draw(turtle: &mut TurtlePlan) {
    const AXIS_LENGTH: f32 = 250.0;
    const GRID_STEP: f32 = 50.0;
    const FONT_SIZE: i32 = 16;
    const SMALL_FONT: i32 = 24;

    // Draw coordinate system center and axes
    turtle
        .reset()
        .set_speed(1100)
        .pen_up()
        .go_to(vec2(0.0, 0.0))
        .pen_down();

    // Draw X axis (horizontal)
    turtle.set_pen_color(macroquad::prelude::BLACK);
    turtle.set_pen_width(2.0);
    turtle.pen_up().go_to(vec2(-AXIS_LENGTH, 0.0)).pen_down();
    turtle.forward(2.0 * AXIS_LENGTH);

    // Draw Y axis (vertical)
    turtle.pen_up().go_to(vec2(0.0, -AXIS_LENGTH)).pen_down();
    turtle.set_heading(90.0);
    turtle.forward(2.0 * AXIS_LENGTH);

    // Draw axis tick marks (major and minor)
    turtle.set_pen_width(1.0);
    const MINOR_STEP: f32 = GRID_STEP / 10.0; // 1/10th markers

    let mut pos = -AXIS_LENGTH;
    while pos <= AXIS_LENGTH {
        if (pos - 0.0).abs() > 1.0 {
            // Skip origin
            // Determine if this is a major or minor tick
            let is_major = (pos / GRID_STEP).abs().fract() < 0.01;
            let tick_length = if is_major { 10.0 } else { 5.0 };

            // X axis tick
            turtle
                .pen_up()
                .set_pen_width(tick_length / 5.0)
                .go_to(vec2(pos, -tick_length / 2.0))
                .pen_down()
                .set_heading(90.0)
                .forward(tick_length);

            // Y axis tick
            turtle
                .pen_up()
                .go_to(vec2(-tick_length / 2.0, pos))
                .set_pen_width(tick_length / 5.0)
                .pen_down()
                .set_heading(0.0)
                .forward(tick_length);
        }
        pos += MINOR_STEP;
    }

    // Label axes
    turtle
        .pen_up()
        .go_to(vec2(AXIS_LENGTH + 20.0, 0.0))
        .set_heading(0.0)
        .write_text("X", FONT_SIZE);

    turtle
        .pen_up()
        .go_to(vec2(0.0, AXIS_LENGTH + 20.0))
        .set_heading(0.0)
        .write_text("Y", FONT_SIZE);

    // Draw and label 4 points (one per quadrant)
    let points = vec![
        (vec2(120.0, 100.0), "A(2|1)"),
        (vec2(-120.0, 100.0), "B(-2|1)"),
        (vec2(-120.0, -100.0), "C(-2|-1)"),
        (vec2(120.0, -100.0), "D(2|-1)"),
    ];

    for (position, label) in points {
        // Draw point as small circle
        turtle
            .pen_up()
            .go_to(position)
            .set_pen_color(macroquad::prelude::RED)
            .set_pen_width(7.0)
            .pen_down()
            .forward(1.0); // Just a small mark

        // Add label
        let label_offset = vec2(label.len() as f32 * 2.5, 0.0);
        turtle
            .pen_up()
            .go_to(position - label_offset)
            .set_heading(0.0)
            .write_text(label, SMALL_FONT);
    }

    // Label origin
    turtle
        .pen_up()
        .go_to(vec2(-30.0, -20.0))
        .set_heading(0.0)
        .write_text("O", FONT_SIZE);
}
