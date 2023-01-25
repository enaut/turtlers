use bevy::prelude::*;
use turtle_lib::builders::{DirectionalMovement, StopLine, Turnable};
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

    let mt = turtle.pen_up();

    //commands.spawn((turtle, Egon {}));
}
