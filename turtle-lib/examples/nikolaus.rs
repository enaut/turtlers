//! Nikolaus example - draws a house-like figure

use turtle_lib::*;

fn nikolausquadrat(turtle: &mut TurtlePlan, groesse: f32) {
    turtle.forward(groesse);
    turtle.left(90.0);
    turtle.forward(groesse);
    turtle.left(90.0);
    turtle.forward(groesse);
    turtle.left(90.0);
    turtle.forward(groesse);
    turtle.left(90.0);
}

fn nikolausdiag(turtle: &mut TurtlePlan, groesse: f32) {
    let quadrat = groesse * groesse;
    let diag = (quadrat + quadrat).sqrt();

    turtle.left(45.0);
    turtle.forward(diag);
    turtle.left(45.0);
    nikolausdach2(turtle, groesse);
    turtle.left(45.0);
    turtle.forward(diag);
    turtle.left(45.0);
}

fn nikolausdach2(turtle: &mut TurtlePlan, groesse: f32) {
    let quadrat = groesse * groesse;
    let diag = (quadrat + quadrat).sqrt();
    turtle.left(45.0);
    turtle.forward(diag / 2.0);
    turtle.left(90.0);
    turtle.forward(diag / 2.0);
    turtle.left(45.0);
}

fn nikolaus(turtle: &mut TurtlePlan, groesse: f32) {
    nikolausquadrat(turtle, groesse);
    nikolausdiag(turtle, groesse);
}

#[turtle_main("Nikolaus")]
fn draw(turtle: &mut TurtlePlan) {
    turtle.shape(ShapeType::Turtle);

    // Position the turtle (pen up, move, pen down)
    turtle.pen_up();
    turtle.backward(80.0);
    turtle.left(90.0);
    turtle.forward(50.0);
    turtle.right(90.0);
    turtle.pen_down();

    nikolaus(turtle, 100.0);
}
