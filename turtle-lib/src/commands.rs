//! Turtle commands and command queue

use crate::general::{AnimationSpeed, Color, Coordinate, Degrees, FontSize, Precision, Radians};
use crate::shapes::TurtleShape;

/// Individual turtle commands
#[derive(Clone, Debug)]
pub enum TurtleCommand {
    // Movement (positive = forward, negative = backward)
    Move(Precision),

    // Rotation (positive = right/clockwise, negative = left/counter-clockwise)
    // Stored in degrees — the natural unit at the user-facing API boundary.
    Turn(Degrees),

    // Circle drawing
    Circle {
        radius: Precision,
        angle: Degrees, // sweep angle — degrees, as supplied by the user
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
    /// Heading stored as internal radians (Y-down render-space convention).
    /// Values passed via `TurtlePlan::set_heading` are converted from
    /// user-facing degrees before this command is enqueued.
    SetHeading(Radians),

    // Visibility
    ShowTurtle,
    HideTurtle,

    // Fill operations
    BeginFill,
    EndFill,

    // Text rendering
    WriteText {
        text: String,
        font_size: FontSize,
    },

    // Reset
    Reset,
}

/// A pure-data sequence of turtle commands.
///
/// `CommandQueue` is intentionally *not* an `Iterator` — it carries no cursor
/// state.  Execution state ("which command are we on?") belongs to the
/// consumer; `TweenController` owns the cursor that walks this queue.
#[derive(Clone, Debug)]
pub struct CommandQueue {
    commands: Vec<TurtleCommand>,
}

impl CommandQueue {
    #[must_use]
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            commands: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, command: TurtleCommand) {
        self.commands.push(command);
    }

    pub fn extend(&mut self, commands: impl IntoIterator<Item = TurtleCommand>) {
        self.commands.extend(commands);
    }

    /// Return a reference to the command at `index`, or `None` if out of range.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&TurtleCommand> {
        self.commands.get(index)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.commands.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }
}

impl Default for CommandQueue {
    fn default() -> Self {
        Self::new()
    }
}

/// Consuming iteration — yields every command in order.
///
/// This is used by `CommandQueue::extend` and `TweenController::append_commands`
/// to drain one queue into another.  It does *not* imply that `CommandQueue`
/// itself is stateful; the cursor always lives in the consumer.
impl IntoIterator for CommandQueue {
    type Item = TurtleCommand;
    type IntoIter = std::vec::IntoIter<TurtleCommand>;

    fn into_iter(self) -> Self::IntoIter {
        self.commands.into_iter()
    }
}
