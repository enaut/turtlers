use std::time::Duration;

use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*, window::close_on_esc};
use bevy_inspector_egui::RegisterInspectable;
use bevy_prototype_lyon::prelude::{Path, ShapePlugin};
use bevy_tweening::{
    component_animator_system, lens::TransformScaleLens, Animator, EaseFunction, Tween,
    TweenCompleted, TweeningPlugin,
};
use events::DrawingStartedEvent;
use shapes::{TurtleColors, TurtleShape};
use turtle_bundle::{AnimatedTurtle, TurtleBundle};

pub use commands::TurtleCommands;

pub mod builders;
mod commands;
mod debug;
mod drawing;
pub mod events;
mod general;
pub mod shapes;
mod state;
pub mod turtle_bundle;

/**
The turtle plugin is the core of this turtle module.

In order to facilitate the setup this plugin also inserts the `DefaultPlugins` and many other things.

Before using any of the functions add this plugin using:
```rust

app::new().add_plugin(turtle_lib::TurtlePlugin)
```
*/
pub struct TurtlePlugin;

impl Plugin for TurtlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Immigration Game".to_string(),
                width: 1200.,
                height: 800.,
                present_mode: bevy::window::PresentMode::Fifo, // vsync
                ..Default::default()
            },
            ..default()
        }))
        .add_plugin(debug::DebugPlugin)
        .add_plugin(ShapePlugin)
        .add_plugin(TweeningPlugin)
        .add_event::<DrawingStartedEvent>()
        .add_startup_system(setup)
        .add_system(keypresses)
        .add_system(component_animator_system::<Path>)
        .add_system(close_on_esc)
        .add_system(draw_lines)
        .register_inspectable::<TurtleColors>()
        .register_inspectable::<TurtleCommands>();
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::BEIGE),
        },
        ..default()
    });
}

pub fn get_a_turtle() -> AnimatedTurtle {
    let animator = Animator::new(Tween::new(
        EaseFunction::QuadraticInOut,
        Duration::from_millis(3000),
        TransformScaleLens {
            start: Vec3::ZERO,
            end: Vec3::ONE * 1.3,
        },
    ));
    let turtle_bundle = TurtleBundle::default();
    AnimatedTurtle {
        animator,
        turtle_bundle,
        turtle_shape: TurtleShape,
    }
}

fn keypresses(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut tcmd: Query<(Entity, &mut TurtleCommands)>,
    mut turtle: Query<&mut Animator<Transform>, With<TurtleShape>>,
    mut ev_start: EventWriter<DrawingStartedEvent>,
) {
    if keys.just_pressed(KeyCode::W) {
        for (entity, mut tcmd) in tcmd.iter_mut() {
            crate::drawing::run_step::run_animation_step(&mut commands, &mut tcmd, &mut turtle);
            ev_start.send(DrawingStartedEvent(entity))
        }
    }
}

fn draw_lines(
    mut commands: Commands,
    mut tcmd: Query<&mut TurtleCommands>,
    mut turtle: Query<&mut Animator<Transform>, With<TurtleShape>>,
    mut query_event: EventReader<TweenCompleted>, // TODO: howto attach only to the right event?
) {
    for _ev in query_event.iter() {
        if let Ok(mut tcmd) = tcmd.get_single_mut() {
            crate::drawing::run_step::run_animation_step(&mut commands, &mut tcmd, &mut turtle)
        } else {
            println!("Failed to get the turtle")
        }
    }
}
