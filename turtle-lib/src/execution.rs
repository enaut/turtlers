//! Command execution logic

use crate::circle_geometry::{CircleDirection, CircleGeometry};
use crate::commands::TurtleCommand;
use crate::state::{DrawCommand, Turtle, TurtleParams, TurtleWorld};
use crate::tessellation;
use macroquad::prelude::*;

#[cfg(test)]
use crate::general::AnimationSpeed;

/// Execute side effects for commands that don't involve movement
/// Returns true if the command was handled (caller should skip movement processing)
#[allow(clippy::too_many_lines)]
pub fn execute_command_side_effects(command: &TurtleCommand, state: &mut Turtle) -> bool {
    match command {
        TurtleCommand::BeginFill => {
            if state.filling.is_some() {
                tracing::warn!(
                    turtle_id = state.turtle_id,
                    "begin_fill() called while already filling"
                );
            }
            let fill_color = state.params.fill_color.unwrap_or_else(|| {
                tracing::warn!(
                    turtle_id = state.turtle_id,
                    "No fill_color set, using black"
                );
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

                let span = tracing::debug_span!(
                    "end_fill",
                    turtle_id = state.turtle_id,
                    contours = fill_state.contours.len()
                );
                let _enter = span.enter();

                for (i, contour) in fill_state.contours.iter().enumerate() {
                    tracing::debug!(
                        turtle_id = state.turtle_id,
                        contour_idx = i,
                        vertices = contour.len(),
                        "Contour info"
                    );
                }

                if !fill_state.contours.is_empty() {
                    if let Ok(mesh_data) = tessellation::tessellate_multi_contour(
                        &fill_state.contours,
                        fill_state.fill_color,
                    ) {
                        tracing::debug!(
                            turtle_id = state.turtle_id,
                            contours = fill_state.contours.len(),
                            "Successfully created fill mesh - persisting to commands"
                        );
                        state.commands.push(DrawCommand::Mesh {
                            data: mesh_data,
                            source: crate::state::TurtleSource {
                                command: crate::commands::TurtleCommand::EndFill,
                                color: state.params.color,
                                fill_color: fill_state.fill_color,
                                pen_width: state.params.pen_width,
                                start_position: fill_state.start_position,
                                end_position: fill_state.start_position,
                            },
                        });
                    } else {
                        tracing::error!(
                            turtle_id = state.turtle_id,
                            "Failed to tessellate contours"
                        );
                    }
                }
            } else {
                tracing::warn!(
                    turtle_id = state.turtle_id,
                    "end_fill() called without begin_fill()"
                );
            }
            true
        }
        TurtleCommand::PenUp => {
            state.params.pen_down = false;
            if state.filling.is_some() {
                tracing::debug!(
                    turtle_id = state.turtle_id,
                    "PenUp: Closing current contour"
                );
            }
            state.close_fill_contour();
            true
        }
        TurtleCommand::PenDown => {
            state.params.pen_down = true;
            if state.filling.is_some() {
                tracing::debug!(
                    turtle_id = state.turtle_id,
                    x = state.params.position.x,
                    y = state.params.position.y,
                    "PenDown: Starting new contour"
                );
            }
            state.start_fill_contour();
            true
        }

        TurtleCommand::Reset => {
            state.reset();
            true
        }

        TurtleCommand::WriteText { text, font_size } => {
            state.commands.push(DrawCommand::Text {
                text: text.clone(),
                position: state.params.position,
                heading: state.params.heading,
                font_size: *font_size,
                color: state.params.color,
                source: crate::state::TurtleSource {
                    command: command.clone(),
                    color: state.params.color,
                    fill_color: state.params.fill_color.unwrap_or(BLACK),
                    pen_width: state.params.pen_width,
                    start_position: state.params.position,
                    end_position: state.params.position,
                },
            });
            true
        }

        TurtleCommand::Move(_)
        | TurtleCommand::Turn(_)
        | TurtleCommand::Circle { .. }
        | TurtleCommand::Goto(_)
        | TurtleCommand::SetColor(_)
        | TurtleCommand::SetFillColor(_)
        | TurtleCommand::SetPenWidth(_)
        | TurtleCommand::SetSpeed(_)
        | TurtleCommand::SetShape(_)
        | TurtleCommand::SetHeading(_)
        | TurtleCommand::ShowTurtle
        | TurtleCommand::HideTurtle => false,
    }
}

/// Record fill vertices after movement commands have updated state
#[tracing::instrument]
pub fn record_fill_vertices_after_movement(
    command: &TurtleCommand,
    start_state: &TurtleParams,
    state: &mut Turtle,
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
#[tracing::instrument]
pub fn execute_command(command: &TurtleCommand, state: &mut Turtle) {
    // Try to execute as side-effect-only command first
    if execute_command_side_effects(command, state) {
        return; // Command fully handled
    }

    // Store start state for fill vertex recording
    let start_state = state.clone();

    // Execute movement and appearance commands
    match command {
        TurtleCommand::Move(distance) => {
            let start = state.params.position;
            let dx = distance * state.params.heading.cos();
            let dy = distance * state.params.heading.sin();
            state.params.position =
                vec2(state.params.position.x + dx, state.params.position.y + dy);

            if state.params.pen_down {
                // Draw line segment with round caps (caps handled by tessellate_stroke)
                if let Ok(mesh_data) = tessellation::tessellate_stroke(
                    &[start, state.params.position],
                    state.params.color,
                    state.params.pen_width,
                    false, // not closed
                ) {
                    state.commands.push(DrawCommand::Mesh {
                        data: mesh_data,
                        source: crate::state::TurtleSource {
                            command: command.clone(),
                            color: state.params.color,
                            fill_color: state.params.fill_color.unwrap_or(BLACK),
                            pen_width: state.params.pen_width,
                            start_position: start,
                            end_position: state.params.position,
                        },
                    });
                }
            }
        }

        TurtleCommand::Turn(degrees) => {
            state.params.heading += degrees.to_radians();
        }

        TurtleCommand::Circle {
            radius,
            angle,
            steps,
            direction,
        } => {
            let start_heading = state.params.heading;
            let geom =
                CircleGeometry::new(state.params.position, start_heading, *radius, *direction);

            if state.params.pen_down {
                // Use Lyon to tessellate the arc
                if let Ok(mesh_data) = tessellation::tessellate_arc(
                    geom.center,
                    *radius,
                    geom.start_angle_from_center.to_degrees(),
                    *angle,
                    state.params.color,
                    state.params.pen_width,
                    *steps,
                    *direction,
                ) {
                    state.commands.push(DrawCommand::Mesh {
                        data: mesh_data,
                        source: crate::state::TurtleSource {
                            command: command.clone(),
                            color: state.params.color,
                            fill_color: state.params.fill_color.unwrap_or(BLACK),
                            pen_width: state.params.pen_width,
                            start_position: state.params.position,
                            end_position: state.params.position,
                        },
                    });
                }
            }

            // Update turtle position and heading
            state.params.position = geom.position_at_angle(angle.to_radians());
            state.params.heading = match direction {
                CircleDirection::Left => start_heading - angle.to_radians(),
                CircleDirection::Right => start_heading + angle.to_radians(),
            };
        }

        TurtleCommand::Goto(coord) => {
            let start = state.params.position;
            // Flip Y coordinate: turtle graphics uses Y+ = up, but Macroquad uses Y+ = down
            state.params.position = vec2(coord.x, -coord.y);

            if state.params.pen_down {
                // Draw line segment with round caps
                if let Ok(mesh_data) = tessellation::tessellate_stroke(
                    &[start, state.params.position],
                    state.params.color,
                    state.params.pen_width,
                    false, // not closed
                ) {
                    state.commands.push(DrawCommand::Mesh {
                        data: mesh_data,
                        source: crate::state::TurtleSource {
                            command: command.clone(),
                            color: state.params.color,
                            fill_color: state.params.fill_color.unwrap_or(BLACK),
                            pen_width: state.params.pen_width,
                            start_position: start,
                            end_position: state.params.position,
                        },
                    });
                }
            }
        }

        // Appearance commands
        TurtleCommand::SetColor(color) => state.params.color = *color,
        TurtleCommand::SetFillColor(color) => state.params.fill_color = *color,
        TurtleCommand::SetPenWidth(width) => state.params.pen_width = *width,
        TurtleCommand::SetSpeed(speed) => state.set_speed(*speed),
        TurtleCommand::SetShape(shape) => state.params.shape = shape.clone(),
        TurtleCommand::SetHeading(heading) => state.params.heading = *heading,
        TurtleCommand::ShowTurtle => state.params.visible = true,
        TurtleCommand::HideTurtle => state.params.visible = false,

        // Reset
        TurtleCommand::Reset => {
            state.reset();
        }

        _ => {} // Already handled by execute_command_side_effects
    }

    // Record fill vertices AFTER movement
    record_fill_vertices_after_movement(command, &start_state.params, state);
}

/// Execute command on a specific turtle by ID
pub fn execute_command_with_id(command: &TurtleCommand, turtle_id: usize, world: &mut TurtleWorld) {
    // Clone turtle state to avoid borrow checker issues
    if let Some(turtle) = world.get_turtle(turtle_id) {
        let mut state = turtle.clone();
        execute_command(command, &mut state);
        // Update the turtle state back
        if let Some(turtle_mut) = world.get_turtle_mut(turtle_id) {
            *turtle_mut = state;
        }
    }
}

/// Add drawing command for a completed tween
pub fn add_draw_for_completed_tween(
    command: &TurtleCommand,
    start_state: &TurtleParams,
    end_state: &mut TurtleParams,
) -> Option<DrawCommand> {
    match command {
        TurtleCommand::Move(_) | TurtleCommand::Goto(_) => {
            if start_state.pen_down {
                if let Ok(mesh_data) = tessellation::tessellate_stroke(
                    &[start_state.position, end_state.position],
                    start_state.color,
                    start_state.pen_width,
                    false,
                ) {
                    return Some(DrawCommand::Mesh {
                        data: mesh_data,
                        source: crate::state::TurtleSource {
                            command: command.clone(),
                            color: start_state.color,
                            fill_color: start_state.fill_color.unwrap_or(BLACK),
                            pen_width: start_state.pen_width,
                            start_position: start_state.position,
                            end_position: end_state.position,
                        },
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
                    return Some(DrawCommand::Mesh {
                        data: mesh_data,
                        source: crate::state::TurtleSource {
                            command: command.clone(),
                            color: start_state.color,
                            fill_color: start_state.fill_color.unwrap_or(BLACK),
                            pen_width: start_state.pen_width,
                            start_position: start_state.position,
                            end_position: end_state.position,
                        },
                    });
                }
            }
        }
        _ => (),
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::TurtleCommand;
    use crate::shapes::TurtleShape;
    use crate::TweenController;

    #[test]
    fn test_forward_left_forward() {
        // Test that after forward(100), left(90), forward(50)
        // the turtle ends up at (100, -50) from initial position (0, 0)
        use crate::state::TurtleParams;

        let state = Turtle {
            turtle_id: 0,
            params: TurtleParams {
                position: vec2(0.0, 0.0),
                heading: 0.0,
                pen_down: false, // Disable drawing to avoid needing TurtleWorld
                pen_width: 1.0,
                color: Color::new(0.0, 0.0, 0.0, 1.0),
                fill_color: None,
                visible: true,
                shape: TurtleShape::turtle(),
                speed: AnimationSpeed::Instant(100),
            },
            filling: None,
            commands: Vec::new(),
            tween_controller: TweenController::default(),
        };

        // We'll use a dummy world but won't actually call drawing commands
        let world = TurtleWorld {
            turtles: vec![state.clone()],
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
        assert_eq!(state.params.position.x, 0.0);
        assert_eq!(state.params.position.y, 0.0);
        assert_eq!(state.params.heading, 0.0);

        // Forward 100 - should move to (100, 0)
        execute_command(&TurtleCommand::Move(100.0), &mut state);
        assert!(
            (state.params.position.x - 100.0).abs() < 0.01,
            "After forward(100): x = {}",
            state.params.position.x
        );
        assert!(
            (state.params.position.y - 0.0).abs() < 0.01,
            "After forward(100): y = {}",
            state.params.position.y
        );
        assert!((state.params.heading - 0.0).abs() < 0.01);

        // Left 90 degrees - should face north (heading decreases by 90°)
        // In screen coords: north = -90° = -π/2
        execute_command(&TurtleCommand::Turn(-90.0), &mut state);
        assert!(
            (state.params.position.x - 100.0).abs() < 0.01,
            "After left(90): x = {}",
            state.params.position.x
        );
        assert!(
            (state.params.position.y - 0.0).abs() < 0.01,
            "After left(90): y = {}",
            state.params.position.y
        );
        let expected_heading = -90.0f32.to_radians();
        assert!(
            (state.params.heading - expected_heading).abs() < 0.01,
            "After left(90): heading = {} (expected {})",
            state.params.heading,
            expected_heading
        );

        // Forward 50 - should move north (negative Y) to (100, -50)
        execute_command(&TurtleCommand::Move(50.0), &mut state);
        assert!(
            (state.params.position.x - 100.0).abs() < 0.01,
            "Final position: x = {} (expected 100.0)",
            state.params.position.x
        );
        assert!(
            (state.params.position.y - (-50.0)).abs() < 0.01,
            "Final position: y = {} (expected -50.0)",
            state.params.position.y
        );
    }
}
