//! Breadboard circuit diagram example.
//!
//! Demonstrates structured procedural drawing of an electronics solderless breadboard.
//! Run normally to display on screen, or export to SVG with:
//! `cargo run --package turtle-lib --example breadboard --features svg -- --export-svg breadboard.svg`

use turtle_lib::*;

fn pin(t: &mut TurtlePlan, size: f32) {
    t.left(90.0).forward(size / 2.0);
    for _ in 0..5 {
        t.right(90.0).forward(size);
    }
    t.right(90.0).forward(size / 2.0).left(90.0);
}

fn pin_row(t: &mut TurtlePlan, count: usize) {
    for x in 0..count {
        pin(t, 5.0);
        if x < count - 1 {
            t.forward(5.0);
        }
    }
}

fn pin_column(t: &mut TurtlePlan, count: usize, x_coord: f32) {
    for x in 0..count {
        t.pen_up().go_to(vec2(x_coord, x as f32 * 10.0)).pen_down();
        pin_row(t, 5);
    }
}

fn pin_side(t: &mut TurtlePlan, count: usize, x_coord: f32, color: Color) {
    t.pen_up()
        .go_to(vec2(x_coord, -2.5))
        .pen_down()
        .set_pen_color(color)
        .set_heading(90.0);
    for x in 0..count {
        pin(t, 5.0);
        if x < count - 1 {
            t.forward(5.0);
        }
    }
}

fn draw_breadboard(t: &mut TurtlePlan, row_count: usize) {
    pin_column(t, row_count, 0.0);
    pin_column(t, row_count, 65.0);
    pin_side(t, row_count, -15.0, BLUE);
    pin_side(t, row_count, -25.0, RED);
    pin_side(t, row_count, 125.0, BLUE);
    pin_side(t, row_count, 135.0, RED);

    // draw outline
    t.pen_up().go_to(vec2(-30.0, -5.0)).pen_down();
    t.set_pen_color(BLACK)
        .forward(row_count as f32 * 10.0 + 10.0)
        .right(90.0)
        .forward(170.0)
        .right(90.0)
        .forward(row_count as f32 * 10.0 + 10.0)
        .right(90.0)
        .forward(170.0)
        .right(90.0);
}

#[turtle_main("Breadboard")]
fn main(turtle: &mut TurtlePlan) {
    turtle.set_speed(1200).set_pen_width(0.5);
    draw_breadboard(turtle, 65);
    turtle.hide();
}
