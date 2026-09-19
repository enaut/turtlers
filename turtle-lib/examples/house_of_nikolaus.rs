//! House of Nikolaus example - draws the classic house figure (Eulerian path puzzle)

use turtle_lib::*;

fn house_square(turtle: &mut TurtlePlan, size: f32) {
    turtle.forward(size);
    turtle.left(90.0);
    turtle.forward(size);
    turtle.left(90.0);
    turtle.forward(size);
    turtle.left(90.0);
    turtle.forward(size);
    turtle.left(90.0);
}

fn house_diagonal(turtle: &mut TurtlePlan, size: f32) {
    let square = size * size;
    let diag = (square + square).sqrt();

    turtle.left(45.0);
    turtle.forward(diag);
    turtle.left(45.0);
    house_roof(turtle, size);
    turtle.left(45.0);
    turtle.forward(diag);
    turtle.left(45.0);
}

fn house_roof(turtle: &mut TurtlePlan, size: f32) {
    let square = size * size;
    let diag = (square + square).sqrt();
    turtle.left(45.0);
    turtle.forward(diag / 2.0);
    turtle.left(90.0);
    turtle.forward(diag / 2.0);
    turtle.left(45.0);
}

fn house_of_nikolaus(turtle: &mut TurtlePlan, size: f32) {
    house_square(turtle, size);
    house_diagonal(turtle, size);
}

#[turtle_main("House of Nikolaus")]
fn draw(turtle: &mut TurtlePlan) {
    turtle.shape(ShapeType::Turtle);

    // Position the turtle (pen up, move, pen down)
    turtle.pen_up();
    turtle.backward(80.0);
    turtle.left(90.0);
    turtle.backward(50.0);
    turtle.right(90.0);
    turtle.pen_down();

    house_of_nikolaus(turtle, 100.0);
}
