use bevy::prelude::*;
use turtle_lib::builders::{CurvedMovement, DirectionalMovement};
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
    turtle.set_speed(1000);
    stern(&mut turtle);

    commands.spawn((turtle, Egon {}));
}

fn stern(turtle: &mut AnimatedTurtle) {
    for _ in 0..5 {
        turtle.forward(200);
        turtle.circle(10, 72);
        turtle.circle_right(5, 360);
        turtle.circle(10, 72);
    }
}
