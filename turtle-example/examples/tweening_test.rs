use bevy::prelude::*;

// Note: bevy_inspector_egui and bevy_tweening are not yet fully compatible with Bevy 0.17
// This example is disabled until they are updated
// use bevy_inspector_egui::prelude::*;
// use bevy_tweening::{lens::*, *};

fn main() {
    App::default()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "TransformPositionLens".to_string(),
                resolution: (1400, 600).into(),
                present_mode: bevy::window::PresentMode::Fifo, // vsync
                ..default()
            }),
            ..default()
        }))
        // .add_systems(Update, bevy::window::close_on_esc)
        // .add_plugins(TweeningPlugin)
        .add_systems(Startup, setup)
        // .add_systems(Update, update_animation_speed)
        .register_type::<Options>()
        .run();
}

#[derive(Copy, Clone, PartialEq, Reflect, Resource)]
struct Options {
    speed: f32,
}

impl Default for Options {
    fn default() -> Self {
        Self { speed: 1. }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // NOTE: This example is disabled because bevy_tweening is not yet compatible with Bevy 0.17
    // Once bevy_tweening is updated, uncomment the code below
    
    /*
    let size = 25.;

    let spacing = 1.5;
    let screen_x = 570.;
    let screen_y = 150.;
    let mut x = -screen_x;

    for ease_function in &[
        EaseFunction::QuadraticIn,
        EaseFunction::QuadraticOut,
        EaseFunction::QuadraticInOut,
        EaseFunction::CubicIn,
        EaseFunction::CubicOut,
        EaseFunction::CubicInOut,
        EaseFunction::QuarticIn,
        EaseFunction::QuarticOut,
        EaseFunction::QuarticInOut,
        EaseFunction::QuinticIn,
        EaseFunction::QuinticOut,
        EaseFunction::QuinticInOut,
        EaseFunction::SineIn,
        EaseFunction::SineOut,
        EaseFunction::SineInOut,
        EaseFunction::CircularIn,
        EaseFunction::CircularOut,
        EaseFunction::CircularInOut,
        EaseFunction::ExponentialIn,
        EaseFunction::ExponentialOut,
        EaseFunction::ExponentialInOut,
        EaseFunction::ElasticIn,
        EaseFunction::ElasticOut,
        EaseFunction::ElasticInOut,
        EaseFunction::BackIn,
        EaseFunction::BackOut,
        EaseFunction::BackInOut,
        EaseFunction::BounceIn,
        EaseFunction::BounceOut,
        EaseFunction::BounceInOut,
    ] {
        let tween = Tween::new(
            *ease_function,
            std::time::Duration::from_secs(1),
            TransformPositionLens {
                start: Vec3::new(x, screen_y, 0.),
                end: Vec3::new(x, -screen_y, 0.),
            },
        )
        .then(
            Tween::new(
                EaseFunction::QuadraticInOut,
                std::time::Duration::from_millis(200),
                TransformRotateZLens {
                    start: 0.05,
                    end: -0.05,
                },
            )
            .with_repeat_strategy(bevy_tweening::RepeatStrategy::MirroredRepeat)
            .with_repeat_count(RepeatCount::Infinite),
        );

        commands.spawn((
            Sprite {
                color: Color::srgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(size, size)),
                ..default()
            },
            Animator::new(tween),
        ));

        x += size * spacing;
    }
    */
}

fn update_animation_speed(_options: Res<Options> /*, mut animators: Query<&mut Animator<Transform>>*/) {
    // NOTE: This function is disabled because bevy_tweening is not yet compatible with Bevy 0.17
    /*
    if !options.is_changed() {
        return;
    }

    for mut animator in animators.iter_mut() {
        animator.set_speed(options.speed);
    }
    */
}
