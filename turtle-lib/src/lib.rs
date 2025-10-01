use bevy::{prelude::*, window::WindowResolution};
#[cfg(feature = "tweening")]
use bevy_prototype_lyon::entity::Shape;
use bevy_prototype_lyon::prelude::ShapePlugin;
#[cfg(feature = "tweening")]
use bevy_tweening::{
    component_animator_system, lens::TransformScaleLens, Animator, EaseFunction, Tween,
    TweenCompleted,
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
            primary_window: Some(Window {
                title: "Immigration Game".to_string(),
                resolution: WindowResolution::new(1200, 800),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(debug::DebugPlugin)
        .add_plugins(ShapePlugin)
        .add_message::<DrawingStartedEvent>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                keypresses,
                #[cfg(feature = "tweening")]
                component_animator_system::<Transform>,
                #[cfg(feature = "tweening")]
                component_animator_system::<Shape>,
                #[cfg(feature = "tweening")]
                draw_lines,
                #[cfg(not(feature = "tweening"))]
                draw_immediate,
            ),
        )
        .register_type::<TurtleColors>()
        .register_type::<TurtleCommands>();
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d,));
}

pub fn get_a_turtle() -> AnimatedTurtle {
    #[cfg(feature = "tweening")]
    let animator = Animator::new(Tween::new(
        EaseFunction::QuadraticInOut,
        Duration::from_millis(3000),
        TransformScaleLens {
            start: Vec3::ZERO,
            end: Vec3::ONE * 1.3,
        },
    ));
    #[cfg(not(feature = "tweening"))]
    let animator = ();
    let turtle_bundle = TurtleBundle::default();
    AnimatedTurtle {
        animator,
        turtle_bundle,
        turtle_shape: TurtleShape,
    }
}

fn keypresses(
    mut commands: Commands,
    _keys: Res<ButtonInput<KeyCode>>,
    mut tcmd: Query<(Entity, &mut TurtleCommands)>,
    #[cfg(feature = "tweening")] mut turtle: Query<&mut Animator<Transform>, With<TurtleShape>>,
    #[cfg(not(feature = "tweening"))] mut turtle: Query<&mut Transform, With<TurtleShape>>,
    mut _ev_start: MessageWriter<DrawingStartedEvent>,
) {
    if _keys.just_pressed(KeyCode::KeyW) {
        for (entity, mut tcmd) in tcmd.iter_mut() {
            #[cfg(feature = "tweening")]
            crate::drawing::run_step::run_animation_step(&mut commands, &mut tcmd, &mut turtle);
            
            #[cfg(not(feature = "tweening"))]
            crate::drawing::immediate::run_all_commands_immediately(&mut commands, &mut tcmd, &mut turtle);
            
            _ev_start.write(DrawingStartedEvent(entity));
        }
    }
}

#[cfg(feature = "tweening")]
fn draw_lines(
    mut commands: Commands,
    mut tcmd: Query<&mut TurtleCommands>,
    mut turtle: Query<&mut Animator<Transform>, With<TurtleShape>>,
    mut query_event: EventReader<TweenCompleted>, // TODO: howto attach only to the right event?
) {
    for _ev in query_event.read() {
        if let Ok(mut tcmd) = tcmd.get_single_mut() {
            crate::drawing::run_step::run_animation_step(&mut commands, &mut tcmd, &mut turtle)
        } else {
            println!("Failed to get the turtle")
        }
    }
}

#[cfg(not(feature = "tweening"))]
fn draw_immediate(
    mut commands: Commands,
    mut tcmd: Query<&mut TurtleCommands>,
    mut turtle: Query<&mut Transform, With<TurtleShape>>,
) {
    for mut tcmd in tcmd.iter_mut() {
        // Only draw if there are commands to execute
        if tcmd.animation_state() < tcmd.commands().len() {
            crate::drawing::immediate::run_all_commands_immediately(&mut commands, &mut tcmd, &mut turtle);
        }
    }
}
