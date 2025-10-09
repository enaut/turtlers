//! Turtle commands and command queue

use crate::general::{Color, Coordinate, Precision};
use crate::shapes::TurtleShape;

/// Individual turtle commands
#[derive(Clone, Debug)]
pub enum TurtleCommand {
    // Movement
    Forward(Precision),
    Backward(Precision),

    // Rotation
    Left(Precision),  // degrees
    Right(Precision), // degrees

    // Circle drawing
    CircleLeft {
        radius: Precision,
        angle: Precision, // degrees
        steps: usize,
    },
    CircleRight {
        radius: Precision,
        angle: Precision, // degrees
        steps: usize,
    },

    // Pen control
    PenUp,
    PenDown,

    // Appearance
    SetColor(Color),
    SetFillColor(Option<Color>),
    SetPenWidth(Precision),
    SetSpeed(u32),
    SetShape(TurtleShape),

    // Position
    Goto(Coordinate),
    SetHeading(Precision), // radians

    // Visibility
    ShowTurtle,
    HideTurtle,
}

/// Queue of turtle commands with execution state
#[derive(Debug)]
pub struct CommandQueue {
    commands: Vec<TurtleCommand>,
    current_index: usize,
}

impl CommandQueue {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
            current_index: 0,
        }
    }

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

    pub fn next(&mut self) -> Option<&TurtleCommand> {
        if self.current_index < self.commands.len() {
            let cmd = &self.commands[self.current_index];
            self.current_index += 1;
            Some(cmd)
        } else {
            None
        }
    }

    pub fn is_complete(&self) -> bool {
        self.current_index >= self.commands.len()
    }

    pub fn reset(&mut self) {
        self.current_index = 0;
    }

    pub fn len(&self) -> usize {
        self.commands.len()
    }

    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    pub fn remaining(&self) -> usize {
        self.commands.len().saturating_sub(self.current_index)
    }
}

impl Default for CommandQueue {
    fn default() -> Self {
        Self::new()
    }
}
