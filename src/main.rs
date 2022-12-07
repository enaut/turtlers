use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use turtle_lib::builders::Turnable;
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
    let mut p = turtle.create_plan();
    for x in 0..1999 {
        p.forward(x.into());
        p.right(45.into());
        p.forward(30.into());
        p.left((90 + x).into());
        p.forward(30.into());
        p.right(45.into());
        p.forward(x.into());
        p.left(91.into());
    }
    turtle.apply_plan(p);
    commands.spawn((turtle, Egon {}));
}
