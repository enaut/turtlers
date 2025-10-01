use bevy::prelude::{Commands, Query, Quat, Transform, Vec2, With};

use crate::{
    commands::{DrawElement, MoveCommand, OrientationCommand, TurtleCommands, TurtleSegment},
    general::angle::Angle,
    shapes::TurtleShape,
};

use super::line_segments::{TurtleDrawCircle, TurtleDrawLine};

/// Executes all turtle commands immediately without animation
pub fn run_all_commands_immediately(
    commands: &mut Commands,
    tcmd: &mut TurtleCommands,
    turtle: &mut Query<&mut Transform, With<TurtleShape>>,
) {
    if let Ok(mut turtle_transform) = turtle.single_mut() {
        // Execute all commands
        loop {
            let idx = tcmd.animation_state();
            if idx >= tcmd.commands().len() {
                break;
            }
            
            let segment = tcmd.commands()[idx].clone();
            match segment {
                TurtleSegment::Single(element) => {
                    execute_draw_element(commands, &element, tcmd, &mut turtle_transform);
                }
                TurtleSegment::Outline(elements) => {
                    for element in elements {
                        execute_draw_element(commands, &element, tcmd, &mut turtle_transform);
                    }
                }
                TurtleSegment::Filled(elements) => {
                    for element in elements {
                        execute_draw_element(commands, &element, tcmd, &mut turtle_transform);
                    }
                }
            }
            *tcmd.animation_state_mut() += 1;
        }
    }
}

fn execute_draw_element(
    commands: &mut Commands,
    element: &DrawElement,
    tcmd: &mut TurtleCommands,
    turtle_transform: &mut Transform,
) {
    match element {
        DrawElement::Draw(move_cmd) => {
            match move_cmd {
                MoveCommand::Forward(length) => {
                    let start = tcmd.state_mut().position();
                    let end = start + (Vec2::from_angle(tcmd.state_mut().heading().to_radians().value()) * length.0);
                    
                    tcmd.state_mut().set_position(end);
                    
                    // Update turtle position
                    turtle_transform.translation.x = end.x;
                    turtle_transform.translation.y = end.y;
                    
                    // Draw line
                    let line = TurtleDrawLine::new(start, end);
                    commands.spawn(line);
                }
                MoveCommand::Backward(length) => {
                    let start = tcmd.state_mut().position();
                    let end = start + (Vec2::from_angle(tcmd.state_mut().heading().to_radians().value()) * -length.0);
                    
                    tcmd.state_mut().set_position(end);
                    
                    // Update turtle position
                    turtle_transform.translation.x = end.x;
                    turtle_transform.translation.y = end.y;
                    
                    // Draw line
                    let line = TurtleDrawLine::new(start, end);
                    commands.spawn(line);
                }
                MoveCommand::Circle { radius, angle } => {
                    let start = tcmd.state_mut().position();
                    let radii = Vec2::ONE * radius.0.abs();
                    let left_right = Angle::degrees(if radius.0 >= 0. { 90. } else { -90. });
                    let heading = tcmd.state_mut().heading();
                    let center = start + (Vec2::new(radius.0.abs(), 0.).rotate(Vec2::from_angle(
                        ((heading + left_right).to_radians()).value(),
                    )));
                    let end_heading = heading + if radius.0 > 0. { *angle } else { -*angle };
                    let end = center + Vec2::new(radius.0.abs(), 0.).rotate(Vec2::from_angle(
                        (heading + *angle - left_right).to_radians().value(),
                    ));
                    
                    tcmd.state_mut().set_position(end);
                    tcmd.state_mut().set_heading(end_heading);
                    
                    // Update turtle position and rotation
                    turtle_transform.translation.x = end.x;
                    turtle_transform.translation.y = end.y;
                    turtle_transform.rotation = Quat::from_rotation_z(end_heading.to_radians().value());
                    
                    // Draw circle arc
                    let circle = TurtleDrawCircle::new(center, radii, *angle, start, end);
                    commands.spawn(circle);
                }
                MoveCommand::Goto(_coord) => {
                    // TODO: implement goto
                }
            }
        }
        DrawElement::Move(move_cmd) => {
            match move_cmd {
                MoveCommand::Forward(length) => {
                    let new_pos = tcmd.state_mut().position() + (Vec2::from_angle(tcmd.state_mut().heading().to_radians().value()) * length.0);
                    tcmd.state_mut().set_position(new_pos);
                    turtle_transform.translation.x = new_pos.x;
                    turtle_transform.translation.y = new_pos.y;
                }
                MoveCommand::Backward(length) => {
                    let new_pos = tcmd.state_mut().position() + (Vec2::from_angle(tcmd.state_mut().heading().to_radians().value()) * -length.0);
                    tcmd.state_mut().set_position(new_pos);
                    turtle_transform.translation.x = new_pos.x;
                    turtle_transform.translation.y = new_pos.y;
                }
                MoveCommand::Circle { .. } => {
                    // TODO: implement move circle
                }
                MoveCommand::Goto(_coord) => {
                    // TODO: implement goto
                }
            }
        }
        DrawElement::Orient(orient_cmd) => {
            match orient_cmd {
                OrientationCommand::Left(angle) => {
                    let new_heading = tcmd.state_mut().heading() - *angle;
                    tcmd.state_mut().set_heading(new_heading);
                    turtle_transform.rotation = Quat::from_rotation_z(new_heading.to_radians().value());
                }
                OrientationCommand::Right(angle) => {
                    let new_heading = tcmd.state_mut().heading() + *angle;
                    tcmd.state_mut().set_heading(new_heading);
                    turtle_transform.rotation = Quat::from_rotation_z(new_heading.to_radians().value());
                }
                OrientationCommand::SetHeading => {
                    // TODO: implement set_heading
                }
                OrientationCommand::LookAt(_coord) => {
                    // TODO: implement look_at
                }
            }
        }
        DrawElement::Drip(_breadcrumb) => {
            // TODO: implement breadcrumbs
        }
    }
}
