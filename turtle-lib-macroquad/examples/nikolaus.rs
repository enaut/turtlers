//! Nikolaus example - draws a house-like figure

use macroquad::prelude::*;
use turtle_lib_macroquad::*;

fn nikolausquadrat(plan: &mut TurtlePlan, groesse: f32) {
    plan.forward(groesse);
    plan.left(90.0);
    plan.forward(groesse);
    plan.left(90.0);
    plan.forward(groesse);
    plan.left(90.0);
    plan.forward(groesse);
    plan.left(90.0);
}

fn nikolausdiag(plan: &mut TurtlePlan, groesse: f32) {
    let quadrat = groesse * groesse;
    let diag = (quadrat + quadrat).sqrt();

    plan.left(45.0);
    plan.forward(diag);
    plan.left(45.0);
    nikolausdach2(plan, groesse);
    plan.left(45.0);
    plan.forward(diag);
    plan.left(45.0);
}

fn nikolausdach2(plan: &mut TurtlePlan, groesse: f32) {
    let quadrat = groesse * groesse;
    let diag = (quadrat + quadrat).sqrt();
    plan.left(45.0);
    plan.forward(diag / 2.0);
    plan.left(90.0);
    plan.forward(diag / 2.0);
    plan.left(45.0);
}

fn nikolaus(plan: &mut TurtlePlan, groesse: f32) {
    nikolausquadrat(plan, groesse);
    nikolausdiag(plan, groesse);
}

#[macroquad::main("Nikolaus")]
async fn main() {
    // Create a turtle plan
    let mut plan = create_turtle();
    plan.shape(ShapeType::Turtle);

    // Position the turtle (pen up, move, pen down)
    plan.pen_up();
    plan.backward(80.0);
    plan.left(90.0);
    plan.forward(50.0);
    plan.right(90.0);
    plan.pen_down();

    nikolaus(&mut plan, 100.0);

    // Create turtle app with animation (speed = 100 pixels/sec)
    let mut app = TurtleApp::new().with_commands(plan.build(), 100.0);

    // Main loop
    loop {
        clear_background(WHITE);

        // Update and render
        app.update();
        app.render();

        next_frame().await
    }
}
