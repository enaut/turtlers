use bevy::prelude::*;
use turtle_lib::builders::{DirectionalMovement, Turnable};
use turtle_lib::turtle_bundle::AnimatedTurtle;
use turtle_lib::{get_a_turtle, TurtlePlugin};

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

#[derive(Component, Reflect)]
struct Egon {}

fn main() {
    App::new()
        .add_plugins(TurtlePlugin)
        .add_systems(Startup, setup)
        //.add_systems(Update, plan)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .run();
}

fn setup(mut commands: Commands) {
    let mut turtle = get_a_turtle();
    turtle.set_speed(500);
    stern(&mut turtle);
    commands.spawn((turtle, Egon {}));
}

fn stern(turtle: &mut AnimatedTurtle) {
    // Draw the roof of the house
    turtle.left(45);
    turtle.forward(70);
    turtle.right(90);
    turtle.forward(70);
    turtle.left(45);

    // Draw the sides of the house
    turtle.left(90);
    turtle.forward(100);
    turtle.left(90);
    turtle.forward(50);
    turtle.left(90);
    turtle.forward(100);
    turtle.left(90);
    turtle.forward(50);

    // Draw the door of the house
    turtle.left(90);
    turtle.forward(25);
    turtle.right(90);
    turtle.forward(20);
    turtle.right(90);
    turtle.forward(25);
    turtle.left(90);
    turtle.forward(50);

    // Draw the chimney of the house
    turtle.left(90);
    turtle.forward(25);
    turtle.right(90);
    turtle.forward(10);
    turtle.right(90);
    turtle.forward(25);
    turtle.left(90);
    turtle.forward(10);
    turtle.left(90);
    turtle.forward(10);
    turtle.right(90);
    turtle.forward(25);
    turtle.left(90);
    turtle.forward(10);
    turtle.left(90);
    turtle.forward(10);
}
