use crate::{
    commands::{DrawElement, TurtleSegment},
    general::{angle::Angle, length::Length, Precision},
};

#[derive(Default, Debug)]
pub struct TurtlePlan {
    commands: Vec<TurtleSegment>,
}

pub trait WithCommands {
    fn get_mut_commands(&mut self) -> &mut Vec<TurtleSegment>;
    fn get_commands(self) -> Vec<TurtleSegment>;
}

impl WithCommands for TurtlePlan {
    fn get_mut_commands(&mut self) -> &mut Vec<TurtleSegment> {
        &mut self.commands
    }

    fn get_commands(self) -> Vec<TurtleSegment> {
        self.commands
    }
}

impl TurtlePlan {
    pub fn new() -> TurtlePlan {
        TurtlePlan { commands: vec![] }
    }
    pub fn forward(&mut self, length: Length) {
        self.get_mut_commands()
            .push(TurtleSegment::Single(DrawElement::Draw(
                crate::commands::MoveCommand::Forward(length),
            )))
    }
    pub fn backward(&mut self, length: Precision) {
        self.get_mut_commands()
            .push(TurtleSegment::Single(DrawElement::Draw(
                crate::commands::MoveCommand::Backward(Length(length)),
            )))
    }
}

pub trait Turnable: WithCommands {
    fn right(&mut self, angle: Angle<Precision>) -> &mut Self {
        self.get_mut_commands()
            .push(TurtleSegment::Single(DrawElement::Orient(
                crate::commands::OrientationCommand::Right(angle),
            )));
        self
    }
    fn left(&mut self, angle: Angle<Precision>) -> &mut Self {
        self.get_mut_commands()
            .push(TurtleSegment::Single(DrawElement::Orient(
                crate::commands::OrientationCommand::Left(angle),
            )));
        self
    }
}

impl Turnable for TurtlePlan {}
