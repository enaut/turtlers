use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
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
    turtle.forward(10.);
    turtle.forward(10.);
    turtle.forward(10.);
    turtle.forward(10.);
    turtle.forward(10.);
    turtle.forward(10.);
    commands.spawn((turtle, Egon {}));
}
