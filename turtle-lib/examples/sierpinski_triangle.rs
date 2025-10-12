//! Draws a Sierpiński triangle with automatic positioning and sizing.
//!
//! The Sierpiński triangle is a fairly simple self-similar fractal geometric shape: it consists of
//! many nested equilateral triangles. More formally, such a triangle is itself three triangles of
//! one level below and a size divided by two. Level zero means a simple equilateral triangle. The
//! drawing procedure is as follows, for a given level and size:
//!
//!  * If level is 0
//!    * Draw an equilateral triangle of the given size.
//!  * otherwise
//!    * Draw the half-sized level - 1 triangle at the bottom left.
//!    * Go the start of the bottom-right slot.
//!    * Draw a half-sized level - 1 triangle.
//!    * Go to the start of the top slot.
//!    * Draw a half-sized level - 1 triangle.
//!
//! That is relatively easy to implement, as long as you follow these steps and let recursion do
//! the rest. Another little bonus this example provides is the ability to customize the drawing
//! size: the triangle will stay correctly sized and positioned automatically.

use macroquad::window::{screen_height, screen_width};
use turtle_lib::*;

/// The number of levels to draw following the recursive procedure.
const LEVELS: u8 = 9;
/// Triangle size (adjust to fit nicely in window)
const TRIANGLE_SIZE: f32 = 300.0;

#[turtle_main("Sierpiński Triangle")]
fn draw_sierpinski(turtle: &mut TurtlePlan) {
    turtle.set_speed(1500); // Fast drawing
    turtle.set_pen_width(0.2);

    // Auto-sized procedure
    sierpinski_triangle_auto(turtle, LEVELS);

    // Hide turtle when done drawing in order to fully reveal the result
    turtle.hide();
}

/// Recursive function drawing a Sierpiński triangle.
///
/// It will do it with the given `turtle` and start at its current position and heading. `level`
/// is the depth of the drawing to be done, zero meaning a simple triangle. `size` is the length
/// of the outermost triangle's sides.
fn sierpinski_triangle(turtle: &mut TurtlePlan, level: u8, size: f32) {
    // When level 0 is reached, just draw an equilateral triangle.
    if level == 0 {
        turtle.pen_down();

        for _ in 0..3 {
            turtle.forward(size);
            turtle.left(120.0);
        }

        turtle.pen_up();
    } else {
        // Parameters for subsequent calls are the same.
        let next_level = level - 1;
        let next_size = size / 2.0;

        // Bottom-left triangle.
        sierpinski_triangle(turtle, next_level, next_size);

        turtle.forward(next_size);

        // Bottom-right triangle.
        sierpinski_triangle(turtle, next_level, next_size);

        turtle.left(120.0);
        turtle.forward(next_size);
        turtle.right(120.0);

        // Top triangle.
        sierpinski_triangle(turtle, next_level, next_size);

        // Go back to the start.
        turtle.right(120.0);
        turtle.forward(next_size);
        turtle.left(120.0);
    }
}

/// Draws a Sierpiński triangle with automatic size and start point.
///
/// `level` is still required, it can't be computed automatically. However, given the used
/// canvas size, it will compute the appropriate size and start point so the triangle gets
/// centered and occupies as much drawing space as possible while staying in bounds.
fn sierpinski_triangle_auto(turtle: &mut TurtlePlan, level: u8) {
    let size = TRIANGLE_SIZE;

    turtle.pen_up();
    turtle.go_to((-screen_width() / 2.0 + 20.0, screen_height() / 2.0 - 20.0));
    turtle.set_heading(0.0); // 0 = East (pointing right)

    // The drawing itself.
    sierpinski_triangle(turtle, level, size);
}
