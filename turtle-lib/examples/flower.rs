//! Draws a simple geometric sort of flower with customizable dimensions.
//!
//! This example makes extensive use of the turtle arc methods: circle_left and circle_right.
//! Ported from the turtle crate example.

use turtle_lib::*;

const BOTTOM_MARGIN: f32 = 25.0;

const LEAF_FILL_COLOR: Color = Color {
    r: 0.0,
    g: 0.5,
    b: 0.0,
    a: 1.0,
}; // green
const LEAF_BORDER_COLOR: Color = Color {
    r: 0.0,
    g: 0.39,
    b: 0.0,
    a: 1.0,
}; // dark green
const LEAF_BORDER_WIDTH: f32 = 1.0;
const LEFT_LEAF_RADIUS: f32 = 200.0;
const LEFT_LEAF_EXTENT: f32 = 45.0;
const RIGHT_LEAF_INCLINATION: f32 = 15.0;
const RIGHT_LEAF_BOTTOM_RADIUS: f32 = 250.0;
const RIGHT_LEAF_BOTTOM_EXTENT: f32 = 45.0;
const RIGHT_LEAF_TOP_RADIUS: f32 = 157.0;
const RIGHT_LEAF_TOP_EXTENT: f32 = 75.0;

const TRUNK_COLOR: Color = LEAF_BORDER_COLOR;
const TRUNK_WIDTH: f32 = 3.0;
const TRUNK_PIECE_COUNT: usize = 4;
const TRUNK_PIECE_RADIUS: f32 = 500.0;
const TRUNK_PIECE_EXTENT: f32 = 15.0;

const PETALS_COUNT: usize = 4;
const PETALS_FILL_COLOR: Color = Color {
    r: 0.5,
    g: 0.0,
    b: 0.5,
    a: 1.0,
}; // purple
const PETALS_BORDER_COLOR: Color = Color {
    r: 0.55,
    g: 0.0,
    b: 0.55,
    a: 1.0,
}; // dark purple
const PETALS_BORDER_WIDTH: f32 = LEAF_BORDER_WIDTH;
const PETALS_INIT_LEFT: f32 = 65.0;
const PETALS_SIDE_RADIUS: f32 = 80.0;
const PETALS_SIDE_EXTENT: f32 = 90.0;
const PETALS_SPACE_GAP: f32 = 20.0;
const PETALS_SPACE_RADIUS: f32 = 40.0;
const PETALS_SPACE_EXTENT: f32 = 30.0;

#[turtle_main("Flower")]
fn draw(turtle: &mut TurtlePlan) {
    // Initial positioning - bottom left area
    turtle
        .pen_up()
        .go_to(vec2(-100.0, -200.0 + BOTTOM_MARGIN))
        .left(90.0)
        .pen_down();

    // Setup
    turtle
        .set_fill_color(LEAF_FILL_COLOR)
        .set_pen_color(LEAF_BORDER_COLOR);

    for _ in 0..TRUNK_PIECE_COUNT {
        // Leaves
        turtle
            .set_pen_width(LEAF_BORDER_WIDTH)
            .set_pen_color(LEAF_BORDER_COLOR)
            .begin_fill();

        // Left leaf
        turtle
            .circle_left(LEFT_LEAF_RADIUS, LEFT_LEAF_EXTENT, 45)
            .right(LEFT_LEAF_EXTENT)
            .circle_right(LEFT_LEAF_RADIUS, -LEFT_LEAF_EXTENT, 45)
            .right(LEFT_LEAF_EXTENT);

        // Right leaf
        turtle.right(RIGHT_LEAF_INCLINATION);

        // Note: circle_left with negative radius is same as circle_right
        // Using circle_right with negative extent instead
        turtle
            .circle_right(RIGHT_LEAF_BOTTOM_RADIUS, RIGHT_LEAF_BOTTOM_EXTENT, 45)
            .right(RIGHT_LEAF_INCLINATION)
            .circle_right(RIGHT_LEAF_TOP_RADIUS, -RIGHT_LEAF_TOP_EXTENT, 75);

        // Trunk piece
        turtle
            .end_fill()
            .set_pen_width(TRUNK_WIDTH)
            .set_pen_color(TRUNK_COLOR)
            .circle_right(TRUNK_PIECE_RADIUS, TRUNK_PIECE_EXTENT, 50);
    }

    // Petals
    turtle
        .set_fill_color(PETALS_FILL_COLOR)
        .set_pen_color(PETALS_BORDER_COLOR)
        .set_pen_width(PETALS_BORDER_WIDTH)
        .left(PETALS_INIT_LEFT)
        .begin_fill()
        .circle_right(PETALS_SIDE_RADIUS, PETALS_SIDE_EXTENT, 90);

    for _ in 0..PETALS_COUNT {
        turtle
            .left(PETALS_SPACE_GAP)
            .circle_right(PETALS_SPACE_RADIUS, -PETALS_SPACE_EXTENT, 30)
            .right(2.0 * PETALS_SPACE_GAP + PETALS_SPACE_EXTENT)
            .circle_left(PETALS_SPACE_RADIUS, PETALS_SPACE_EXTENT, 30);
    }

    // Finish petals with error adjustments
    turtle
        .left(PETALS_SPACE_GAP)
        .circle_left(PETALS_SIDE_RADIUS + 1.0, 3.0 - PETALS_SIDE_EXTENT, 87)
        .end_fill();
}
