//! Dashed circle example ported from sunjay/turtle
//! This draws a dashed circle but uses `circle_left` arcs for each segment instead of individual short lines.

use turtle_lib::{turtle_main, vec2, CurvedMovement, Turnable};

#[turtle_main("Dashed Circle")]
fn draw(turtle: &mut TurtlePlan) {
    // Circle parameters
    let radius = 120.0_f32;
    let number_of_dashes = 24;
    let segments_angle = 360 / number_of_dashes;
    let steps_per_arc = 6; // number of steps to tessellate each small arc

    // Position turtle at circle start
    turtle
        .reset()
        .pen_up()
        .go_to(vec2(radius, 0.0))
        .pen_down()
        .left(90.0);

    for i in 0..number_of_dashes {
        // Determine whether current group is drawing or skipping
        let draw_segment = usize::is_multiple_of(i, 2);

        if draw_segment {
            turtle.pen_down();
        } else {
            turtle.pen_up();
        }

        // Draw a small arc using circle_left. Each call advances the heading by segment_angle.
        turtle.circle_left(radius, segments_angle as f32, steps_per_arc);
    }
}
