use bevy::prelude::Component;
use bevy_inspector_egui::Inspectable;

use crate::{
    drawing::{
        self,
        animation::{
            draw_straight_segment, move_straight_segment, turtle_turn, ToAnimationSegment,
            TurtleAnimationSegment,
        },
        TurtleGraphElement,
    },
    general::{angle::Angle, length::Length, Coordinate, Precision, Speed},
    state::TurtleState,
};
/**
 * All the possibilities to draw something with turtle. All the commands can get the position, heading,
 * color and fill_color from the turtles state.
 */
#[derive(Component, Inspectable, Debug)]
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

#[derive(Component, Inspectable, Default, Debug)]
pub enum Breadcrumb {
    Dot,
    #[default]
    Stamp,
}

/// Different ways that change the orientation of the turtle.
#[derive(Component, Inspectable, Debug)]
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
#[derive(Component, Inspectable, Debug)]
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
impl ToAnimationSegment for DrawElement {
    fn to_draw_segment(
        &self,
        state: &mut TurtleState,
    ) -> crate::drawing::animation::TurtleAnimationSegment {
        match self {
            DrawElement::Draw(e) => match e {
                MoveCommand::Forward(length) => draw_straight_segment(state, length.0),
                MoveCommand::Backward(length) => draw_straight_segment(state, -length.0),
                MoveCommand::Circle { radius, angle } => todo!(),
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

#[derive(Component, Inspectable, Debug)]
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
#[derive(Component, Inspectable, Debug)]
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
}

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
