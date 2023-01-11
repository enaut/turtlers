use std::ops::Neg;

use crate::{
    commands::{DrawElement, MoveCommand, TurtleSegment},
    general::{angle::Angle, length::Length, Precision},
};

#[derive(Default, Debug)]
pub struct TurtlePlan {
    /**
     * A turtle Plan contains the segments of a turtle drawing.
     * The segments in turn contain the commands to draw the graph.
     */
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

pub trait CurvedMovement: WithCommands {
    fn circle<IntoAngle, IntoDistance>(
        &mut self,
        radius: IntoDistance,
        extend: IntoAngle,
    ) -> &mut Self
    where
        Angle<Precision>: From<IntoAngle>,
        Length: From<IntoDistance>,
    {
        let angle: Angle<Precision> = extend.into();
        let radius: Length = radius.into();
        self.get_mut_commands()
            .push(TurtleSegment::Single(DrawElement::Draw(
                MoveCommand::Circle { radius, angle },
            )));
        self
    }
    fn circle_right<IntoAngle, IntoDistance: Neg + Neg<Output = IntoDistance>>(
        &mut self,
        radius: IntoDistance,
        extend: IntoAngle,
    ) -> &mut Self
    where
        Angle<Precision>: From<IntoAngle>,
        Length: From<IntoDistance>,
    {
        self.circle(-radius, extend);
        println!("Warning: circle with right arc not working yet...");
        self
    }
}

impl CurvedMovement for TurtlePlan {}

pub trait StopLine<T>
where
    T: WithCommands,
{
    fn pen_up(self) -> InvisibleLinesPlan<T>;
}

impl StopLine<TurtlePlan> for TurtlePlan {
    fn pen_up(self) -> InvisibleLinesPlan<TurtlePlan> {
        {
            InvisibleLinesPlan {
                before: self,
                commands: vec![],
            }
        }
    }
}

pub trait StartLine<T> {
    fn pen_down(self) -> T;
}

impl<T> StartLine<T> for InvisibleLinesPlan<T>
where
    T: WithCommands,
{
    fn pen_down(mut self) -> T {
        self.before.get_mut_commands().append(&mut self.commands);
        self.before
    }
}

pub struct InvisibleLinesPlan<T: WithCommands> {
    before: T,
    commands: Vec<TurtleSegment>,
}

impl<T> WithCommands for InvisibleLinesPlan<T>
where
    T: WithCommands,
{
    fn get_mut_commands(&mut self) -> &mut Vec<TurtleSegment> {
        &mut self.commands
    }

    fn get_commands(self) -> Vec<TurtleSegment> {
        self.commands
    }
}

impl<T> InvisibleLinesPlan<T>
where
    T: WithCommands,
{
    pub fn new(before: T) -> Self {
        InvisibleLinesPlan {
            before,
            commands: vec![],
        }
    }
}

impl Turnable for InvisibleLinesPlan<TurtlePlan> {}

impl<T> DirectionalMovement for InvisibleLinesPlan<T>
where
    T: WithCommands,
{
    fn forward<IntoDistance>(&mut self, length: IntoDistance) -> &mut Self
    where
        Length: From<IntoDistance>,
    {
        let length: Length = length.into();
        self.get_mut_commands()
            .push(TurtleSegment::Single(DrawElement::Move(
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
            .push(TurtleSegment::Single(DrawElement::Move(
                crate::commands::MoveCommand::Backward(length),
            )));
        self
    }
}

impl<T> CurvedMovement for InvisibleLinesPlan<T>
where
    T: WithCommands,
{
    fn circle<IntoAngle, IntoDistance>(
        &mut self,
        radius: IntoDistance,
        extend: IntoAngle,
    ) -> &mut Self
    where
        Angle<Precision>: From<IntoAngle>,
        Length: From<IntoDistance>,
    {
        let angle: Angle<Precision> = extend.into();
        let radius: Length = radius.into();
        self.get_mut_commands()
            .push(TurtleSegment::Single(DrawElement::Move(
                MoveCommand::Circle { radius, angle },
            )));
        self
    }

    fn circle_right<IntoAngle, IntoDistance: Neg + Neg<Output = IntoDistance>>(
        &mut self,
        radius: IntoDistance,
        extend: IntoAngle,
    ) -> &mut Self
    where
        Angle<Precision>: From<IntoAngle>,
        Length: From<IntoDistance>,
    {
        self.circle(-radius, extend);
        println!("Warning: circle with right arc not working yet...");
        self
    }
}
