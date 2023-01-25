use bevy::prelude::*;
use turtle_lib::builders::{DirectionalMovement, Turnable};
use turtle_lib::turtle_bundle::AnimatedTurtle;
use turtle_lib::{get_a_turtle, TurtlePlugin};

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

#[derive(Component, Reflect)]
struct Egon {}

fn main() {
    App::new()
        .add_plugin(TurtlePlugin)
        .add_startup_system(setup)
        //.add_system(plan)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .run();
}

fn setup(mut commands: Commands) {
    let mut turtle = get_a_turtle();
    turtle.set_speed(1);
    for _x in 0..3 {
        koch(3, &mut turtle);
        turtle.right(120);
    }
    commands.spawn((turtle, Egon {}));
}

fn koch(depth: u32, turtle: &mut AnimatedTurtle) {
    if depth == 0 {
        turtle.forward(10);
    } else {
        koch(depth - 1, turtle);
        turtle.left(60);
        koch(depth - 1, turtle);
        turtle.right(120);
        koch(depth - 1, turtle);
        turtle.left(60);
        koch(depth - 1, turtle);
    }
}
