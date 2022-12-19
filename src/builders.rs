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
}

pub trait DirectionalMovement: WithCommands {
    fn forward<IntoDistance>(&mut self, length: IntoDistance) -> &mut Self
    where
        Length: From<IntoDistance>,
    {
        let length: Length = length.into();
        self.get_mut_commands()
            .push(TurtleSegment::Single(DrawElement::Draw(
                crate::commands::MoveCommand::Forward(length),
            )));
        self
    }
    fn backward<IntoDistance>(&mut self, length: IntoDistance) -> &mut Self
    where
        Length: From<IntoDistance>,
    {
        let length: Length = length.into();
        self.get_mut_commands()
            .push(TurtleSegment::Single(DrawElement::Draw(
                crate::commands::MoveCommand::Backward(length),
            )));
        self
    }
}

impl DirectionalMovement for TurtlePlan {}

pub trait Turnable: WithCommands {
    fn right<IntoAngle>(&mut self, angle: IntoAngle) -> &mut Self
    where
        Angle<Precision>: From<IntoAngle>,
    {
        let angle: Angle<Precision> = angle.into();
        self.get_mut_commands()
            .push(TurtleSegment::Single(DrawElement::Orient(
                crate::commands::OrientationCommand::Right(angle),
            )));
        self
    }
    fn left<IntoAngle>(&mut self, angle: IntoAngle) -> &mut Self
    where
        Angle<Precision>: From<IntoAngle>,
    {
        let angle: Angle<Precision> = angle.into();
        self.get_mut_commands()
            .push(TurtleSegment::Single(DrawElement::Orient(
                crate::commands::OrientationCommand::Left(angle),
            )));
        self
    }
}

impl Turnable for TurtlePlan {}
