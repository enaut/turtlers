//! Draw a dragon curve, more specifically a Heighway dragon.
//!
//! (https://en.wikipedia.org/wiki/Dragon_curve)
//!
//! As can be seen in the above Wikipedia article, the Heighway dragon can be
//! constructed by repeatedly folding a strip of paper and looking at the
//! directions of the folds/turns.
//!
//! Starting with a strip going left to right (l2r):
//!
//! start|--->---l2r--->---|end
//!
//! you might fold it like this:
//!
//! end|---<---r2l---<---\
//! start|->---l2r--->---/
//!
//! Getting a l2r strip, followed by a left turn, followed by a r2l strip.
//!
//! Folding a right to left strip:
//!
//! end|---<---r2l---<---|start
//!
//! In the same way:
//!
//! start|-->---l2r--->---\
//! end|----<---r2l---<---/
//!
//! Would give you a l2r, followed by a right turn, followed by a r2l strip.
//!
//! As you can see, the only difference between the two is the direction of
//! the turn in the middle.
//!
//! This folding of paper is simulated by recursively calling the dragon(..)
//! function, passing the direction of the turn for this fold as an angle
//! (+90 for a right turn, -90 for a left turn).

use turtle_lib_macroquad::*;

#[turtle_main("Dragon Curve")]
fn draw_dragon(turtle: &mut TurtlePlan) {
    // Fast drawing
    turtle.set_speed(1200);

    // Start position
    turtle.pen_up();
    turtle.backward(160.0);
    turtle.right(90.0);
    turtle.forward(110.0);
    turtle.pen_down();
    turtle.set_pen_width(6.);

    // Draw the dragon curve with 13 folds
    dragon(turtle, -90.0, 13, 0.0, 255.0);

    // Hide turtle when done
    turtle.hide();
}

/// Draw the dragon curve by simulating folding a strip of paper
///
/// Arguments:
/// `fold_direction`: The direction of the fold, +90 for a right, -90 for a
/// left turn.
/// `num_folds`: The number of times to fold the 'strip of paper'.
/// `color_start`/`color_end`: The color at the start/end of this subsection
/// of the curve as a number 0-255.
fn dragon(
    turtle: &mut TurtlePlan,
    fold_direction: f32,
    num_folds: usize,
    color_start: f32,
    color_end: f32,
) {
    let color_mid = (color_start + color_end) * 0.5;

    if num_folds == 0 {
        // Mapping a color number 0-255 to an RGB gradient
        let red = ((color_mid - 128.0).abs() * 2.0).floor();
        let green = color_mid;
        let blue = 160.0;

        turtle.set_pen_color(Color::new(red / 255.0, green / 255.0, blue / 255.0, 1.0));
        turtle.forward(10.0);
        return;
    }

    // Draw a left to right strip (which has a left turn in the middle)
    dragon(turtle, -90.0, num_folds - 1, color_start, color_mid);
    turtle.right(fold_direction);
    // Draw a right to left strip (which has a right turn in the middle)
    dragon(turtle, 90.0, num_folds - 1, color_mid, color_end);
}
