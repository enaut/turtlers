//! Turtle commands and command queue

use crate::general::{AnimationSpeed, Color, Coordinate, Precision};
use crate::shapes::TurtleShape;

/// Individual turtle commands
#[derive(Clone, Debug)]
pub enum TurtleCommand {
    // Movement (positive = forward, negative = backward)
    Move(Precision),

    // Rotation (positive = right/clockwise, negative = left/counter-clockwise in degrees)
    Turn(Precision),

    // Circle drawing
    Circle {
        radius: Precision,
        angle: Precision, // degrees
        steps: usize,
        direction: crate::circle_geometry::CircleDirection,
    },

    // Pen control
    PenUp,
    PenDown,

    // Appearance
    SetColor(Color),
    SetFillColor(Option<Color>),
    SetPenWidth(Precision),
    SetSpeed(AnimationSpeed),
    SetShape(TurtleShape),

    // Position
    Goto(Coordinate),
    SetHeading(Precision), // radians

    // Visibility
    ShowTurtle,
    HideTurtle,

    // Fill operations
    BeginFill,
    EndFill,
}

/// Queue of turtle commands with execution state
#[derive(Clone, Debug)]
pub struct CommandQueue {
    commands: Vec<TurtleCommand>,
    current_index: usize,
}

impl CommandQueue {
    #[must_use]
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            current_index: 0,
        }
    }
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            commands: Vec::with_capacity(capacity),
            current_index: 0,
        }
    }

    pub fn push(&mut self, command: TurtleCommand) {
        self.commands.push(command);
    }

    pub fn extend(&mut self, commands: impl IntoIterator<Item = TurtleCommand>) {
        self.commands.extend(commands);
    }
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.current_index >= self.commands.len()
    }
    pub fn reset(&mut self) {
        self.current_index = 0;
    }
    #[must_use]
    pub fn len(&self) -> usize {
        self.commands.len()
    }
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    #[must_use]
    pub fn remaining(&self) -> usize {
        self.commands.len().saturating_sub(self.current_index)
    }
}

impl Default for CommandQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for CommandQueue {
    type Item = TurtleCommand;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index < self.commands.len() {
            let cmd = self.commands[self.current_index].clone();
            self.current_index += 1;
            Some(cmd)
        } else {
            None
        }
    }
}
