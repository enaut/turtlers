use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use turtle_lib::builders::{DirectionalMovement, Turnable};
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
    turtle.set_speed(1);
    for _x in 0..3 {
        koch(4, &mut turtle);
        let mut left = turtle.create_plan();
        left.right(120);
        turtle.extend_plan(left);
    }
    commands.spawn((turtle, Egon {}));
}

fn koch(depth: u32, turtle: &mut AnimatedTurtle) {
    if depth == 0 {
        let mut forward = turtle.create_plan();
        forward.forward(10);
        turtle.extend_plan(forward)
    } else {
        koch(depth - 1, turtle);
        let mut left = turtle.create_plan();
        left.left(60);
        turtle.extend_plan(left);
        koch(depth - 1, turtle);
        let mut right = turtle.create_plan();
        right.right(120);
        turtle.extend_plan(right);
        koch(depth - 1, turtle);
        let mut left = turtle.create_plan();
        left.left(60);
        turtle.extend_plan(left);
        koch(depth - 1, turtle);
    }
}
