//! Koch snowflake fractal example

use macroquad::prelude::*;
use turtle_lib_macroquad::*;

fn koch(depth: u32, plan: &mut TurtlePlan, distance: f32) {
    if depth == 0 {
        plan.forward(distance);
    } else {
        let new_distance = distance / 3.0;
        koch(depth - 1, plan, new_distance);
        plan.left(60.0);
        koch(depth - 1, plan, new_distance);
        plan.right(120.0);
        koch(depth - 1, plan, new_distance);
        plan.left(60.0);
        koch(depth - 1, plan, new_distance);
    }
}

#[macroquad::main("Koch Snowflake")]
async fn main() {
    let mut plan = create_turtle();

    // Position turtle
    plan.set_speed(1001);
    plan.pen_up();
    plan.backward(150.0);

    plan.pen_down();

    // Draw Koch snowflake (triangle of Koch curves)
    for _ in 0..3 {
        koch(4, &mut plan, 300.0);
        plan.right(120.0);
        plan.set_speed(1200);
    }

    plan.hide(); // Hide turtle when done

    // Create app with animation
    let mut app = TurtleApp::new().with_commands(plan.build());

    loop {
        clear_background(WHITE);
        app.update();
        app.render();
        next_frame().await
    }
}
