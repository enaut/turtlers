//! Builder pattern traits for creating turtle command sequences

use crate::commands::{CommandQueue, TurtleCommand};
use crate::general::{Color, Precision};
use crate::shapes::{ShapeType, TurtleShape};

/// Trait for adding commands to a queue
pub trait WithCommands {
    fn get_commands_mut(&mut self) -> &mut CommandQueue;
    fn get_commands(self) -> CommandQueue;
}

/// Trait for forward/backward movement
pub trait DirectionalMovement: WithCommands {
    fn forward<T>(&mut self, distance: T) -> &mut Self
    where
        T: Into<Precision>,
    {
        let dist: Precision = distance.into();
        self.get_commands_mut().push(TurtleCommand::Forward(dist));
        self
    }

    fn backward<T>(&mut self, distance: T) -> &mut Self
    where
        T: Into<Precision>,
    {
        let dist: Precision = distance.into();
        self.get_commands_mut().push(TurtleCommand::Backward(dist));
        self
    }
}

/// Trait for turning operations
pub trait Turnable: WithCommands {
    fn left<T>(&mut self, angle: T) -> &mut Self
    where
        T: Into<Precision>,
    {
        let degrees: Precision = angle.into();
        self.get_commands_mut().push(TurtleCommand::Left(degrees));
        self
    }

    fn right<T>(&mut self, angle: T) -> &mut Self
    where
        T: Into<Precision>,
    {
        let degrees: Precision = angle.into();
        self.get_commands_mut().push(TurtleCommand::Right(degrees));
        self
    }
}

/// Trait for curved movement (circles)
pub trait CurvedMovement: WithCommands {
    fn circle_left<R, A>(&mut self, radius: R, angle: A, steps: usize) -> &mut Self
    where
        R: Into<Precision>,
        A: Into<Precision>,
    {
        let r: Precision = radius.into();
        let a: Precision = angle.into();
        self.get_commands_mut().push(TurtleCommand::CircleLeft {
            radius: r,
            angle: a,
            steps,
        });
        self
    }

    fn circle_right<R, A>(&mut self, radius: R, angle: A, steps: usize) -> &mut Self
    where
        R: Into<Precision>,
        A: Into<Precision>,
    {
        let r: Precision = radius.into();
        let a: Precision = angle.into();
        self.get_commands_mut().push(TurtleCommand::CircleRight {
            radius: r,
            angle: a,
            steps,
        });
        self
    }
}

/// Builder for creating turtle command sequences
#[derive(Default, Debug)]
pub struct TurtlePlan {
    queue: CommandQueue,
}

impl TurtlePlan {
    pub fn new() -> Self {
        Self {
            queue: CommandQueue::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            queue: CommandQueue::with_capacity(capacity),
        }
    }

    pub fn set_speed(&mut self, speed: u32) -> &mut Self {
        self.queue.push(TurtleCommand::SetSpeed(speed));
        self
    }

    pub fn set_color(&mut self, color: Color) -> &mut Self {
        self.queue.push(TurtleCommand::SetColor(color));
        self
    }

    pub fn set_pen_width(&mut self, width: Precision) -> &mut Self {
        self.queue.push(TurtleCommand::SetPenWidth(width));
        self
    }

    pub fn pen_up(&mut self) -> &mut Self {
        self.queue.push(TurtleCommand::PenUp);
        self
    }

    pub fn pen_down(&mut self) -> &mut Self {
        self.queue.push(TurtleCommand::PenDown);
        self
    }

    pub fn hide(&mut self) -> &mut Self {
        self.queue.push(TurtleCommand::HideTurtle);
        self
    }

    pub fn show(&mut self) -> &mut Self {
        self.queue.push(TurtleCommand::ShowTurtle);
        self
    }

    pub fn set_shape(&mut self, shape: TurtleShape) -> &mut Self {
        self.queue.push(TurtleCommand::SetShape(shape));
        self
    }

    pub fn shape(&mut self, shape_type: ShapeType) -> &mut Self {
        self.set_shape(shape_type.to_shape())
    }

    pub fn build(self) -> CommandQueue {
        self.queue
    }
}

impl WithCommands for TurtlePlan {
    fn get_commands_mut(&mut self) -> &mut CommandQueue {
        &mut self.queue
    }

    fn get_commands(self) -> CommandQueue {
        self.queue
    }
}

impl DirectionalMovement for TurtlePlan {}
impl Turnable for TurtlePlan {}
impl CurvedMovement for TurtlePlan {}
