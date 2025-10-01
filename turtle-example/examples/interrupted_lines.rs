use bevy::prelude::*;
use turtle_lib::builders::{DirectionalMovement, StopLine, Turnable};
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
    turtle.set_speed(1);

    // NOTE: pen_up() consumes self, which is why this example is incomplete
    // TODO: Fix the builder API to work with the deref pattern
    // let mt = turtle.pen_up();

    commands.spawn((turtle, Egon {}));
}
