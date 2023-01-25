use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use turtle_lib::builders::{CurvedMovement, DirectionalMovement, Turnable};
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
    turtle.set_speed(1000);
    turtle.circle(50, 90);
    turtle.circle_right(50, 180);
    turtle.circle(50, 90);
    for x in 0..1999 {
        turtle.forward(x);
        turtle.right(45);
        turtle.forward(30);
        turtle.left(90 + x);
        turtle.forward(30);
        turtle.right(45);
        turtle.forward(x);
        turtle.left(91);
    }
    commands.spawn((turtle, Egon {}));
}
