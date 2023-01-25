use bevy::prelude::{Commands, Query, Transform, With};
use bevy_tweening::Animator;

use crate::{commands::TurtleCommands, shapes::TurtleShape};

use super::{animation::TurtleAnimationSegment, TurtleGraphElement};

pub fn run_animation_step(
    commands: &mut Commands,
    tcmd: &mut TurtleCommands,
    turtle: &mut Query<&mut Animator<Transform>, With<TurtleShape>>,
) {
    loop {
        match tcmd.next() {
            Some(TurtleAnimationSegment {
                turtle_animation: Some(turtle_animation),
                line_segment: Some(graph_element_to_draw),
                line_animation: Some(line_animation),
            }) => {
                let mut turtle = turtle.single_mut();
                turtle.set_tweenable(turtle_animation);
                match graph_element_to_draw {
                    TurtleGraphElement::TurtleLine(line) => {
                        commands.spawn((line, line_animation));
                    }
                    TurtleGraphElement::Noop => (),
                    TurtleGraphElement::TurtleCircle(circle) => {
                        commands.spawn((circle, line_animation));
                    }
                }
                return;
            }
            // In case a rotation is performed the line drawing can be skipped
            Some(TurtleAnimationSegment {
                turtle_animation: Some(turtle_animation),
                line_segment: Some(_),
                line_animation: None,
            })|
            // In case a rotation is performed the line drawing can be skipped
            Some(TurtleAnimationSegment {
                turtle_animation: Some(turtle_animation),
                line_segment: None,
                line_animation: None,
            }) => {
                let mut turtle = turtle.single_mut();
                turtle.set_tweenable(turtle_animation);
                return;
            }
            Some(_e) => {
                println!("without animation");
            }
            None => {
                println!("nothing to draw");
                return;
            }
        };
    }
}
