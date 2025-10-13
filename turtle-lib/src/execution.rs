//! Command execution logic

use crate::circle_geometry::{CircleDirection, CircleGeometry};
use crate::commands::TurtleCommand;
use crate::state::{DrawCommand, TurtleState, TurtleWorld};
use crate::tessellation;
use macroquad::prelude::*;

#[cfg(test)]
use crate::general::AnimationSpeed;

/// Execute side effects for commands that don't involve movement
/// Returns true if the command was handled (caller should skip movement processing)
pub fn execute_command_side_effects(
    command: &TurtleCommand,
    state: &mut TurtleState,
    commands: &mut Vec<DrawCommand>,
) -> bool {
    match command {
        TurtleCommand::BeginFill => {
            if state.filling.is_some() {
                tracing::warn!("begin_fill() called while already filling");
            }
            let fill_color = state.fill_color.unwrap_or_else(|| {
                tracing::warn!("No fill_color set, using black");
                BLACK
            });
            state.begin_fill(fill_color);
            true
        }

        TurtleCommand::EndFill => {
            if let Some(mut fill_state) = state.filling.take() {
                if !fill_state.current_contour.is_empty() {
                    fill_state.contours.push(fill_state.current_contour);
                }

                let span = tracing::debug_span!("end_fill", contours = fill_state.contours.len());
                let _enter = span.enter();

                for (i, contour) in fill_state.contours.iter().enumerate() {
                    tracing::debug!(contour_idx = i, vertices = contour.len(), "Contour info");
                }

                if !fill_state.contours.is_empty() {
                    if let Ok(mesh_data) = tessellation::tessellate_multi_contour(
                        &fill_state.contours,
                        fill_state.fill_color,
                    ) {
                        tracing::debug!(
                            contours = fill_state.contours.len(),
                            "Successfully tessellated contours"
                        );
                        commands.push(DrawCommand::Mesh {
                            turtle_id: 0,
                            data: mesh_data,
                        });
                    } else {
                        tracing::error!("Failed to tessellate contours");
                    }
                }
            } else {
                tracing::warn!("end_fill() called without begin_fill()");
            }
            true
        }

        TurtleCommand::PenUp => {
            state.pen_down = false;
            if state.filling.is_some() {
                tracing::debug!("PenUp: Closing current contour");
            }
            state.close_fill_contour();
            true
        }

        TurtleCommand::PenDown => {
            state.pen_down = true;
            if state.filling.is_some() {
                tracing::debug!(
                    x = state.position.x,
                    y = state.position.y,
                    "PenDown: Starting new contour"
                );
            }
            state.start_fill_contour();
            true
        }

        _ => false, // Not a side-effect-only command
    }
}

/// Record fill vertices after movement commands have updated state
pub fn record_fill_vertices_after_movement(
    command: &TurtleCommand,
    start_state: &TurtleState,
    state: &mut TurtleState,
) {
    if state.filling.is_none() {
        return;
    }

    match command {
        TurtleCommand::Circle {
            radius,
            angle,
            steps,
            direction,
        } => {
            let geom = CircleGeometry::new(
                start_state.position,
                start_state.heading,
                *radius,
                *direction,
            );
            state.record_fill_vertices_for_arc(
                geom.center,
                *radius,
                geom.start_angle_from_center,
                angle.to_radians(),
                *direction,
                *steps as u32,
            );
        }
        TurtleCommand::Move(_) | TurtleCommand::Goto(_) => {
            state.record_fill_vertex();
        }
        _ => {}
    }
}

/// Execute a single turtle command, updating state and adding draw commands
pub fn execute_command(command: &TurtleCommand, state: &mut TurtleState, world: &mut TurtleWorld) {
    // Try to execute as side-effect-only command first
    if execute_command_side_effects(command, state, &mut world.commands) {
        return; // Command fully handled
    }

    // Store start state for fill vertex recording
    let start_state = state.clone();

    // Execute movement and appearance commands
    match command {
        TurtleCommand::Move(distance) => {
            let start = state.position;
            let dx = distance * state.heading.cos();
            let dy = distance * state.heading.sin();
            state.position = vec2(state.position.x + dx, state.position.y + dy);

            if state.pen_down {
                // Draw line segment with round caps (caps handled by tessellate_stroke)
                if let Ok(mesh_data) = tessellation::tessellate_stroke(
                    &[start, state.position],
                    state.color,
                    state.pen_width,
                    false, // not closed
                ) {
                    world.add_command(DrawCommand::Mesh {
                        turtle_id: 0,
                        data: mesh_data,
                    });
                }
            }
        }

        TurtleCommand::Turn(degrees) => {
            state.heading += degrees.to_radians();
        }

        TurtleCommand::Circle {
            radius,
            angle,
            steps,
            direction,
        } => {
            let start_heading = state.heading;
            let geom = CircleGeometry::new(state.position, start_heading, *radius, *direction);

            if state.pen_down {
                // Use Lyon to tessellate the arc
                if let Ok(mesh_data) = tessellation::tessellate_arc(
                    geom.center,
                    *radius,
                    geom.start_angle_from_center.to_degrees(),
                    *angle,
                    state.color,
                    state.pen_width,
                    *steps,
                    *direction,
                ) {
                    world.add_command(DrawCommand::Mesh {
                        turtle_id: 0,
                        data: mesh_data,
                    });
                }
            }

            // Update turtle position and heading
            state.position = geom.position_at_angle(angle.to_radians());
            state.heading = match direction {
                CircleDirection::Left => start_heading - angle.to_radians(),
                CircleDirection::Right => start_heading + angle.to_radians(),
            };
        }

        TurtleCommand::Goto(coord) => {
            let start = state.position;
            state.position = *coord;

            if state.pen_down {
                // Draw line segment with round caps
                if let Ok(mesh_data) = tessellation::tessellate_stroke(
                    &[start, state.position],
                    state.color,
                    state.pen_width,
                    false, // not closed
                ) {
                    world.add_command(DrawCommand::Mesh {
                        turtle_id: 0,
                        data: mesh_data,
                    });
                }
            }
        }

        // Appearance commands
        TurtleCommand::SetColor(color) => state.color = *color,
        TurtleCommand::SetFillColor(color) => state.fill_color = *color,
        TurtleCommand::SetPenWidth(width) => state.pen_width = *width,
        TurtleCommand::SetSpeed(speed) => state.set_speed(*speed),
        TurtleCommand::SetShape(shape) => state.shape = shape.clone(),
        TurtleCommand::SetHeading(heading) => state.heading = *heading,
        TurtleCommand::ShowTurtle => state.visible = true,
        TurtleCommand::HideTurtle => state.visible = false,

        _ => {} // Already handled by execute_command_side_effects
    }

    // Record fill vertices AFTER movement
    record_fill_vertices_after_movement(command, &start_state, state);
}

/// Execute command on a specific turtle by ID
pub fn execute_command_with_id(command: &TurtleCommand, turtle_id: usize, world: &mut TurtleWorld) {
    // Clone turtle state to avoid borrow checker issues
    if let Some(turtle) = world.get_turtle(turtle_id) {
        let mut state = turtle.clone();
        execute_command(command, &mut state, world);
        // Update the turtle state back
        if let Some(turtle_mut) = world.get_turtle_mut(turtle_id) {
            *turtle_mut = state;
        }
    }
}

/// Add drawing command for a completed tween with turtle_id tracking
pub fn add_draw_for_completed_tween_with_id(
    command: &TurtleCommand,
    start_state: &TurtleState,
    end_state: &TurtleState,
    world: &mut TurtleWorld,
    turtle_id: usize,
) {
    match command {
        TurtleCommand::Move(_) | TurtleCommand::Goto(_) => {
            if start_state.pen_down {
                if let Ok(mesh_data) = tessellation::tessellate_stroke(
                    &[start_state.position, end_state.position],
                    start_state.color,
                    start_state.pen_width,
                    false,
                ) {
                    world.add_command(DrawCommand::Mesh {
                        turtle_id,
                        data: mesh_data,
                    });
                }
            }
        }
        TurtleCommand::Circle {
            radius,
            angle,
            steps,
            direction,
        } => {
            if start_state.pen_down {
                let geom = CircleGeometry::new(
                    start_state.position,
                    start_state.heading,
                    *radius,
                    *direction,
                );
                if let Ok(mesh_data) = tessellation::tessellate_arc(
                    geom.center,
                    *radius,
                    geom.start_angle_from_center.to_degrees(),
                    *angle,
                    start_state.color,
                    start_state.pen_width,
                    *steps,
                    *direction,
                ) {
                    world.add_command(DrawCommand::Mesh {
                        turtle_id,
                        data: mesh_data,
                    });
                }
            }
        }
        _ => {}
    }
}

/// Add drawing command for a completed tween (state transition already occurred)
pub fn add_draw_for_completed_tween(
    command: &TurtleCommand,
    start_state: &TurtleState,
    end_state: &TurtleState,
    world: &mut TurtleWorld,
) {
    match command {
        TurtleCommand::Move(_) | TurtleCommand::Goto(_) => {
            if start_state.pen_down {
                // Draw line segment with round caps
                if let Ok(mesh_data) = tessellation::tessellate_stroke(
                    &[start_state.position, end_state.position],
                    start_state.color,
                    start_state.pen_width,
                    false,
                ) {
                    world.add_command(DrawCommand::Mesh {
                        turtle_id: 0,
                        data: mesh_data,
                    });
                }
            }
        }
        TurtleCommand::Circle {
            radius,
            angle,
            steps,
            direction,
        } => {
            if start_state.pen_down {
                let geom = CircleGeometry::new(
                    start_state.position,
                    start_state.heading,
                    *radius,
                    *direction,
                );

                if let Ok(mesh_data) = tessellation::tessellate_arc(
                    geom.center,
                    *radius,
                    geom.start_angle_from_center.to_degrees(),
                    *angle,
                    start_state.color,
                    start_state.pen_width,
                    *steps,
                    *direction,
                ) {
                    world.add_command(DrawCommand::Mesh {
                        turtle_id: 0,
                        data: mesh_data,
                    });
                }
            }
        }
        _ => {
            // Other commands don't create drawing
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::TurtleCommand;
    use crate::shapes::TurtleShape;

    #[test]
    fn test_forward_left_forward() {
        // Test that after forward(100), left(90), forward(50)
        // the turtle ends up at (100, -50) from initial position (0, 0)
        let mut state = TurtleState {
            position: vec2(0.0, 0.0),
            heading: 0.0,
            pen_down: false, // Disable drawing to avoid needing TurtleWorld
            pen_width: 1.0,
            color: Color::new(0.0, 0.0, 0.0, 1.0),
            fill_color: None,
            speed: AnimationSpeed::Animated(100.0),
            visible: true,
            shape: TurtleShape::turtle(),
            filling: None,
        };

        // We'll use a dummy world but won't actually call drawing commands
        let mut world = TurtleWorld {
            turtles: vec![state.clone()],
            commands: Vec::new(),
            camera: macroquad::camera::Camera2D {
                zoom: vec2(1.0, 1.0),
                target: vec2(0.0, 0.0),
                offset: vec2(0.0, 0.0),
                rotation: 0.0,
                render_target: None,
                viewport: None,
            },
            background_color: Color::new(1.0, 1.0, 1.0, 1.0),
        };
        let mut state = world.turtles[0].clone();

        // Initial state: position (0, 0), heading 0 (east)
        assert_eq!(state.position.x, 0.0);
        assert_eq!(state.position.y, 0.0);
        assert_eq!(state.heading, 0.0);

        // Forward 100 - should move to (100, 0)
        execute_command(&TurtleCommand::Move(100.0), &mut state, &mut world);
        assert!(
            (state.position.x - 100.0).abs() < 0.01,
            "After forward(100): x = {}",
            state.position.x
        );
        assert!(
            (state.position.y - 0.0).abs() < 0.01,
            "After forward(100): y = {}",
            state.position.y
        );
        assert!((state.heading - 0.0).abs() < 0.01);

        // Left 90 degrees - should face north (heading decreases by 90°)
        // In screen coords: north = -90° = -π/2
        execute_command(&TurtleCommand::Turn(-90.0), &mut state, &mut world);
        assert!(
            (state.position.x - 100.0).abs() < 0.01,
            "After left(90): x = {}",
            state.position.x
        );
        assert!(
            (state.position.y - 0.0).abs() < 0.01,
            "After left(90): y = {}",
            state.position.y
        );
        let expected_heading = -90.0f32.to_radians();
        assert!(
            (state.heading - expected_heading).abs() < 0.01,
            "After left(90): heading = {} (expected {})",
            state.heading,
            expected_heading
        );

        // Forward 50 - should move north (negative Y) to (100, -50)
        execute_command(&TurtleCommand::Move(50.0), &mut state, &mut world);
        assert!(
            (state.position.x - 100.0).abs() < 0.01,
            "Final position: x = {} (expected 100.0)",
            state.position.x
        );
        assert!(
            (state.position.y - (-50.0)).abs() < 0.01,
            "Final position: y = {} (expected -50.0)",
            state.position.y
        );
    }
}
