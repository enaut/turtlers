//! Creates colorful geometric art with randomly colored triangles.
//!
//! Ported from the turtle crate example.

use macroquad::prelude::rand;
use turtle_lib::*;

// Parameters to play around with for changing the character of the drawing
const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;
const ROW_COUNT: u32 = 3;
const COL_COUNT: u32 = 4;
const COLOR_COUNT: usize = 7;

/// Generate a random color
fn random_color() -> Color {
    Color {
        r: rand::gen_range(0.0, 1.0),
        g: rand::gen_range(0.0, 1.0),
        b: rand::gen_range(0.0, 1.0),
        a: 1.0,
    }
}

#[turtle_main("Geometric Art")]
fn draw(turtle: &mut TurtlePlan) {
    // Calculate height and width of the triangles
    let row_height = HEIGHT / ROW_COUNT as f32;
    let col_width = WIDTH / COL_COUNT as f32;

    // Prepare a set of random colors to randomly choose from when drawing
    let colors: Vec<Color> = (0..COLOR_COUNT).map(|_| random_color()).collect();

    // Move turtle to the upper left corner of the drawing
    let x_start = -WIDTH / 2.0;
    let y_start = HEIGHT / 2.0;

    turtle.pen_up().go_to(vec2(x_start, y_start)).pen_down();

    // Draw all rows
    for _ in 0..ROW_COUNT {
        // Create an endless loop over the angles of 90 and 270 degrees
        // (corresponds to moving right and left, respectively)
        let mut angle_iter = [90.0, 270.0].iter().cycle();

        // Create triangles from left to right
        for _ in 0..COL_COUNT {
            let angle = *angle_iter.next().unwrap();
            draw_triangle(turtle, angle, col_width, row_height, &colors);
        }

        // Skip one angle so that we have the correct angle when turning around
        angle_iter.next();

        // Fill in triangles from right to left to complete the row
        for _ in 0..COL_COUNT {
            let angle = *angle_iter.next().unwrap();
            draw_triangle(turtle, angle, col_width, row_height, &colors);
        }

        // Reset position to prepare for the next row
        turtle.right(180.0).forward(row_height).left(180.0);
    }
}

/// Draw a single triangle by drawing two sides and filling them with a random color
/// which creates the third side of the triangle along the way.
fn draw_triangle(
    turtle: &mut TurtlePlan,
    angle: f32,
    col_width: f32,
    row_height: f32,
    colors: &[Color],
) {
    // Choose a random color
    let color_index = rand::gen_range(0, colors.len());
    let color = colors[color_index];

    turtle
        .set_fill_color(color)
        .begin_fill()
        .right(angle)
        .forward(col_width)
        .right(angle)
        .forward(row_height)
        .end_fill();
}
