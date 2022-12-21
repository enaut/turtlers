use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use turtle_lib::builders::{CurvedMovement, DirectionalMovement};
use turtle_lib::turtle_bundle::AnimatedTurtle;
use turtle_lib::{get_a_turtle, TurtlePlugin};

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

#[derive(Component, Inspectable)]
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
    turtle.set_speed(500);
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
