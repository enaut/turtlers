use bevy::{prelude::Component, reflect::Reflect};

use crate::{
    builders::WithCommands,
    drawing::TurtleGraphElement,
    general::{angle::Angle, length::Length, Coordinate, Precision, Speed},
    state::TurtleState,
};

#[cfg(feature = "tweening")]
use crate::drawing::animation::{
    draw_circle_segment, draw_straight_segment, move_straight_segment, turtle_turn,
    ToAnimationSegment, TurtleAnimationSegment,
};
/**
 * All the possibilities to draw something with turtle. All the commands can get the position, heading,
 * color and fill_color from the turtles state.
 */
#[derive(Component, Reflect, Debug, Clone)]
pub enum MoveCommand {
    Forward(Length),
    Backward(Length),
    Circle {
        radius: Length,
        angle: Angle<Precision>,
    },
    Goto(Coordinate),
}

impl Default for MoveCommand {
    fn default() -> Self {
        Self::Forward(Length(100.))
    }
}
/// Different ways to drop breadcrumbs on the way like a dot or a stamp of the turtles shape.

#[derive(Component, Reflect, Default, Debug, Clone)]
pub enum Breadcrumb {
    Dot,
    #[default]
    Stamp,
}

/// Different ways that change the orientation of the turtle.
#[derive(Component, Reflect, Debug, Clone)]
pub enum OrientationCommand {
    Left(Angle<Precision>),
    Right(Angle<Precision>),
    SetHeading,
    LookAt(Coordinate),
}

impl Default for OrientationCommand {
    fn default() -> Self {
        Self::Right(Default::default())
    }
}

/// A combination of all commands that can be used while drawing.
#[derive(Component, Reflect, Debug, Clone)]
pub enum DrawElement {
    Draw(MoveCommand),
    Move(MoveCommand),
    Orient(OrientationCommand),
    Drip(Breadcrumb),
}

impl Default for DrawElement {
    fn default() -> Self {
        Self::Draw(Default::default())
    }
}

#[cfg(feature = "tweening")]
impl ToAnimationSegment for DrawElement {
    fn to_draw_segment(
        &self,
        state: &mut TurtleState,
    ) -> crate::drawing::animation::TurtleAnimationSegment {
        match self {
            DrawElement::Draw(e) => match e {
                MoveCommand::Forward(length) => draw_straight_segment(state, length.0),
                MoveCommand::Backward(length) => draw_straight_segment(state, -length.0),
                MoveCommand::Circle { radius, angle } => {
                    draw_circle_segment(state, *radius, *angle)
                }
                MoveCommand::Goto(coord) => todo!(),
            },
            DrawElement::Move(e) => match e {
                MoveCommand::Forward(length) => move_straight_segment(state, length.0),
                MoveCommand::Backward(length) => move_straight_segment(state, -length.0),
                MoveCommand::Circle { radius, angle } => todo!(),
                MoveCommand::Goto(coord) => todo!(),
            },
            DrawElement::Orient(e) => match e {
                OrientationCommand::Left(angle_to_turn) => turtle_turn(state, -*angle_to_turn),
                OrientationCommand::Right(angle_to_turn) => turtle_turn(state, *angle_to_turn),
                OrientationCommand::SetHeading => todo!(),
                OrientationCommand::LookAt(_) => todo!(),
            },
            DrawElement::Drip(_) => todo!(),
        }
    }
}

#[derive(Component, Reflect, Debug, Clone)]
pub enum TurtleSegment {
    Single(DrawElement),
    Outline(Vec<DrawElement>),
    Filled(Vec<DrawElement>),
}

impl Default for TurtleSegment {
    fn default() -> Self {
        Self::Single(Default::default())
    }
}

#[cfg(feature = "tweening")]
impl ToAnimationSegment for TurtleSegment {
    fn to_draw_segment(
        &self,
        state: &mut TurtleState,
    ) -> crate::drawing::animation::TurtleAnimationSegment {
        match self {
            Self::Single(e) => e.to_draw_segment(state),
            Self::Outline(_) => todo!(),
            Self::Filled(_) => todo!(),
        }
    }
}
#[derive(Component, Reflect, Debug)]
pub struct TurtleCommands {
    animation_state: usize,
    commands: Vec<TurtleSegment>,
    lines: Vec<TurtleGraphElement>,
    state: TurtleState,
}

impl TurtleCommands {
    pub fn new(commands: Vec<TurtleSegment>) -> Self {
        let mut state = TurtleState::default();
        state.set_speed(200);
        Self {
            animation_state: 0,
            commands,
            lines: vec![],
            state,
        }
    }
    pub fn push(&mut self, segment: TurtleSegment) {
        self.commands.push(segment)
    }
    pub fn extend(&mut self, segments: Vec<TurtleSegment>) {
        self.commands.extend(segments.into_iter())
    }
    pub fn set_speed(&mut self, speed: Speed) {
        self.state.set_speed(speed);
    }

    // Public accessors for immediate drawing
    pub(crate) fn animation_state(&self) -> usize {
        self.animation_state
    }
    pub(crate) fn animation_state_mut(&mut self) -> &mut usize {
        &mut self.animation_state
    }
    pub(crate) fn commands(&self) -> &[TurtleSegment] {
        &self.commands
    }
    pub(crate) fn state_mut(&mut self) -> &mut TurtleState {
        &mut self.state
    }
}

#[cfg(feature = "tweening")]
impl Iterator for TurtleCommands {
    type Item = TurtleAnimationSegment;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.animation_state;
        let next_index = index + 1;

        if let Some(command) = self.commands.get(self.animation_state) {
            let res = command.to_draw_segment(&mut self.state);
            self.animation_state = next_index;
            Some(res)
        } else {
            None
        }
    }
}

impl WithCommands for TurtleCommands {
    fn get_mut_commands(&mut self) -> &mut Vec<TurtleSegment> {
        &mut self.commands
    }

    fn get_commands(self) -> Vec<TurtleSegment> {
        self.commands
    }
}
