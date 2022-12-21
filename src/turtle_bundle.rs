use std::ops::{Deref, DerefMut};

use bevy::prelude::{Bundle, Color, Name, Transform};
use bevy_prototype_lyon::{
    entity::ShapeBundle,
    prelude::{DrawMode, FillMode, GeometryBuilder, StrokeMode},
};

use crate::{
    builders::{CurvedMovement, DirectionalMovement, Turnable, TurtlePlan, WithCommands},
    commands::{TurtleCommands, TurtleSegment},
    general::Speed,
    shapes::{self, TurtleColors},
};

#[derive(Bundle)]
pub struct TurtleBundle {
    colors: TurtleColors,
    pub commands: TurtleCommands,
    name: Name,
    shape: ShapeBundle,
}

impl Default for TurtleBundle {
    fn default() -> Self {
        Self {
            colors: TurtleColors::default(),
            commands: TurtleCommands::new(vec![]),
            name: Name::new("Turtle"),
            shape: GeometryBuilder::build_as(
                &shapes::turtle(),
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::MIDNIGHT_BLUE),
                    outline_mode: StrokeMode::new(Color::BLACK, 1.0),
                },
                Transform::IDENTITY,
            ),
        }
    }
}

impl TurtleBundle {
    pub fn apply_plan(&mut self, plan: TurtlePlan) {
        self.commands = TurtleCommands::new(plan.get_commands());
    }
    pub fn extend_plan(&mut self, plan: TurtlePlan) {
        self.commands.extend(plan.get_commands())
    }
    pub fn create_plan(&self) -> TurtlePlan {
        TurtlePlan::new()
    }
}

impl TurtleBundle {
    pub fn set_speed(&mut self, speed: Speed) {
        self.commands.set_speed(speed);
    }
}

#[derive(Bundle)]
pub struct AnimatedTurtle {
    pub animator: bevy_tweening::Animator<bevy::prelude::Transform>,
    pub turtle_bundle: TurtleBundle,
    pub turtle_shape: shapes::TurtleShape,
}

impl Deref for AnimatedTurtle {
    type Target = TurtleBundle;

    fn deref(&self) -> &Self::Target {
        &self.turtle_bundle
    }
}

impl DerefMut for AnimatedTurtle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.turtle_bundle
    }
}

impl WithCommands for TurtleBundle {
    fn get_mut_commands(&mut self) -> &mut Vec<TurtleSegment> {
        self.commands.get_mut_commands()
    }

    fn get_commands(self) -> Vec<TurtleSegment> {
        self.commands.get_commands()
    }
}

impl DirectionalMovement for TurtleBundle {}
impl Turnable for TurtleBundle {}
impl CurvedMovement for TurtleBundle {}
