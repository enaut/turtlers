use std::ops::{Deref, DerefMut};

use bevy::prelude::{Bundle, Color, Name};
use bevy_prototype_lyon::{
    draw::{Fill, Stroke},
    entity::Shape,
    geometry::ShapeBuilder,
    prelude::ShapeBuilderBase as _,
};

use crate::{
    builders::{
        CurvedMovement, DirectionalMovement, InvisibleLinesPlan, StopLine, Turnable, TurtlePlan,
        WithCommands,
    },
    commands::{TurtleCommands, TurtleSegment},
    general::Speed,
    shapes::{self, TurtleColors},
};

#[derive(Bundle)]
pub struct TurtleBundle {
    colors: TurtleColors,
    pub commands: TurtleCommands,
    name: Name,
    shape: Shape,
}

impl Default for TurtleBundle {
    fn default() -> Self {
        Self {
            colors: TurtleColors::default(),
            commands: TurtleCommands::new(vec![]),
            name: Name::new("Turtle"),
            shape: ShapeBuilder::with(&shapes::turtle())
                .fill(Fill::color(Color::srgb(0.098, 0.098, 0.439)))
                .stroke(Stroke::new(Color::srgb(0.0, 0.0, 0.0), 1.0))
                .build(),
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
    #[cfg(feature = "tweening")]
    pub animator: bevy_tweening::Animator<bevy::prelude::Transform>,
    #[cfg(not(feature = "tweening"))]
    pub animator: (),
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
impl StopLine<TurtleBundle> for TurtleBundle {
    fn pen_up(self) -> crate::builders::InvisibleLinesPlan<TurtleBundle> {
        {
            InvisibleLinesPlan::new(self)
        }
    }
}

impl DirectionalMovement for TurtleBundle {}
impl Turnable for TurtleBundle {}
impl CurvedMovement for TurtleBundle {}
