//! Command execution logic

use crate::circle_geometry::{CircleDirection, CircleGeometry};
use crate::commands::TurtleCommand;
use crate::general::{Coordinate, Radians};
use crate::state::{DrawCommand, FillState, Turtle, TurtleParams, TurtleWorld};
use crate::tessellation;
use macroquad::prelude::*;

#[cfg(test)]
use crate::general::AnimationSpeed;

/// Close the current open fill contour (factored out of `Turtle::close_fill_contour`).
fn close_fill_contour(turtle_id: usize, filling: &mut Option<FillState>) {
    if let Some(ref mut fill_state) = filling {
        tracing::debug!(
            turtle_id,
            vertices = fill_state.current_contour.len(),
            "close_fill_contour called"
        );
        if fill_state.current_contour.len() >= 2 {
            tracing::debug!(
                turtle_id,
                vertices = fill_state.current_contour.len(),
                first_x = fill_state.current_contour[0].x,
                first_y = fill_state.current_contour[0].y,
                last_x = fill_state.current_contour[fill_state.current_contour.len() - 1].x,
                last_y = fill_state.current_contour[fill_state.current_contour.len() - 1].y,
                "Closing contour"
            );
            let contour = std::mem::take(&mut fill_state.current_contour);
            fill_state.contours.push(contour);
            tracing::debug!(
                turtle_id,
                completed_contours = fill_state.contours.len(),
                "Contour moved to completed list"
            );
        } else if !fill_state.current_contour.is_empty() {
            tracing::warn!(
                turtle_id,
                vertices = fill_state.current_contour.len(),
                "Current contour has insufficient vertices, not closing"
            );
        } else {
            tracing::warn!(turtle_id, "Current contour is empty, nothing to close");
        }
    } else {
        tracing::warn!(
            turtle_id,
            "close_fill_contour called but no active fill state"
        );
    }
}

/// Begin a new fill contour at `position` (factored out of `Turtle::start_fill_contour`).
fn start_fill_contour(turtle_id: usize, position: Coordinate, filling: &mut Option<FillState>) {
    if let Some(ref mut fill_state) = filling {
        tracing::debug!(
            x = position.x,
            y = position.y,
            completed_contours = fill_state.contours.len(),
            turtle_id,
            "Starting new contour"
        );
        fill_state.current_contour = vec![position];
    }
}

/// Execute side effects for commands that don't involve movement.
///
/// Returns `true` if the command was fully handled; the caller should skip
/// params-update and tessellation when this returns `true`.
///
/// Accepts the three logically-separate pieces of turtle state as disjoint
/// mutable borrows so that this function can be called from
/// `TweenController::update(&mut self, …)` without requiring a `&mut Turtle`.
#[allow(clippy::too_many_lines)]
pub(crate) fn execute_command_side_effects(
    command: &TurtleCommand,
    turtle_id: usize,
    params: &mut TurtleParams,
    filling: &mut Option<FillState>,
    commands: &mut Vec<DrawCommand>,
    svg_log: &mut crate::state::SvgLog,
) -> bool {
    match command {
        TurtleCommand::BeginFill => {
            if filling.is_some() {
                tracing::warn!(turtle_id, "begin_fill() called while already filling");
            }
            let fill_color = params.fill_color.unwrap_or_else(|| {
                tracing::warn!(turtle_id, "No fill_color set, using black");
                BLACK
            });
            *filling = Some(FillState {
                start_position: params.position,
                contours: Vec::new(),
                current_contour: vec![params.position],
                fill_color,
            });
            true
        }
        TurtleCommand::EndFill => {
            if let Some(mut fill_state) = filling.take() {
                if !fill_state.current_contour.is_empty() {
                    fill_state.contours.push(fill_state.current_contour);
                }

                let span = tracing::debug_span!(
                    "end_fill",
                    turtle_id,
                    contours = fill_state.contours.len()
                );
                let _enter = span.enter();

                for (i, contour) in fill_state.contours.iter().enumerate() {
                    tracing::debug!(
                        turtle_id,
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
                            turtle_id,
                            contours = fill_state.contours.len(),
                            "Successfully created fill mesh - persisting to commands"
                        );
                        commands.push(DrawCommand::Mesh { data: mesh_data });
                        #[cfg(feature = "svg")]
                        svg_log.push(crate::state::SvgRecord::Fill {
                            contours: fill_state.contours,
                            fill_color: fill_state.fill_color,
                            stroke_color: params.color,
                        });
                    } else {
                        tracing::error!(turtle_id, "Failed to tessellate contours");
                    }
                }
            } else {
                tracing::warn!(turtle_id, "end_fill() called without begin_fill()");
            }
            true
        }
        TurtleCommand::PenUp => {
            params.pen_down = false;
            if filling.is_some() {
                tracing::debug!(turtle_id, "PenUp: Closing current contour");
            }
            close_fill_contour(turtle_id, filling);
            true
        }
        TurtleCommand::PenDown => {
            params.pen_down = true;
            if filling.is_some() {
                tracing::debug!(
                    turtle_id,
                    x = params.position.x,
                    y = params.position.y,
                    "PenDown: Starting new contour"
                );
            }
            start_fill_contour(turtle_id, params.position, filling);
            true
        }

        TurtleCommand::Reset => {
            commands.clear();
            svg_log.clear();
            *filling = None;
            *params = TurtleParams::default();
            true
        }

        TurtleCommand::WriteText { text, font_size } => {
            commands.push(DrawCommand::Text {
                text: text.clone(),
                position: params.position,
                heading: params.heading,
                font_size: *font_size,
                color: params.color,
            });
            #[cfg(feature = "svg")]
            svg_log.push(crate::state::SvgRecord::Text {
                text: text.clone(),
                position: params.position,
                color: params.color,
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

/// Record fill vertices after movement commands have updated state.
///
/// `start_state` is the params snapshot taken **before** the command ran.
/// `params` is the current (post-movement) state — `params.position` is the
/// endpoint that gets pushed into the active fill contour.
///
/// Accepts disjoint borrows so it can be called from `TweenController::update`
/// without needing a `&mut Turtle`.
#[tracing::instrument(skip(params, filling))]
pub(crate) fn record_fill_vertices_after_movement(
    command: &TurtleCommand,
    start_state: &TurtleParams,
    turtle_id: usize,
    params: &TurtleParams,
    filling: &mut Option<FillState>,
) {
    if filling.is_none() {
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
                Radians::new(start_state.heading),
                *radius,
                *direction,
            );
            if let Some(ref mut fill_state) = filling {
                if params.pen_down {
                    let num_samples = (*steps as u32).max(1);
                    tracing::trace!(
                        turtle_id,
                        center_x = geom.center.x,
                        center_y = geom.center.y,
                        radius,
                        steps,
                        num_samples,
                        "Recording arc vertices"
                    );
                    for i in 1..=num_samples {
                        let progress = i as f32 / num_samples as f32;
                        let current_angle = match direction {
                            CircleDirection::Left => {
                                geom.start_angle_from_center - angle.as_radians().value() * progress
                            }
                            CircleDirection::Right => {
                                geom.start_angle_from_center + angle.as_radians().value() * progress
                            }
                        };
                        let vertex = Coordinate::new(
                            geom.center.x + radius * current_angle.cos(),
                            geom.center.y + radius * current_angle.sin(),
                        );
                        tracing::trace!(
                            turtle_id,
                            vertex_idx = i,
                            x = vertex.x,
                            y = vertex.y,
                            angle_degrees = current_angle.to_degrees(),
                            "Arc vertex"
                        );
                        fill_state.current_contour.push(vertex);
                    }
                }
            }
        }
        TurtleCommand::Move(_) | TurtleCommand::Goto(_) => {
            if let Some(ref mut fill_state) = filling {
                if params.pen_down {
                    tracing::trace!(
                        turtle_id,
                        x = params.position.x,
                        y = params.position.y,
                        vertices = fill_state.current_contour.len() + 1,
                        "Adding vertex to current contour"
                    );
                    fill_state.current_contour.push(params.position);
                } else {
                    tracing::trace!(turtle_id, "Skipping vertex (pen is up)");
                }
            }
        }
        _ => {}
    }
}

/// Tessellate a completed movement command into a [`DrawCommand`] mesh.
///
/// Returns `None` if the pen was up or the command does not produce a drawing.
///
/// `end_position` is the turtle's position after the command completed:
/// - instant-mode: `state.params.position` after [`TurtleCommand::apply_to_params`]
/// - animated-mode: `tween.target_params.position` when the tween finishes
///
/// This is the **single** tessellation site for all committed line/arc meshes.
/// It replaces both the inline tessellation inside `execute_command` and the
/// now-deleted `add_draw_for_completed_tween`.
pub(crate) fn tessellate_command(
    command: &TurtleCommand,
    start: &TurtleParams,
    end_position: Vec2,
) -> Option<DrawCommand> {
    if !start.pen_down || !command.produces_drawing() {
        return None;
    }

    match command {
        TurtleCommand::Move(_) | TurtleCommand::Goto(_) => {
            let mesh_data = tessellation::tessellate_stroke(
                &[start.position, end_position],
                start.color,
                start.pen_width,
                false,
            )
            .ok()?;

            Some(DrawCommand::Mesh { data: mesh_data })
        }

        TurtleCommand::Circle {
            radius,
            angle,
            steps,
            direction,
        } => {
            use crate::circle_geometry::CircleGeometry;
            let geom = CircleGeometry::new(
                start.position,
                Radians::new(start.heading),
                *radius,
                *direction,
            );
            let mesh_data = tessellation::tessellate_arc(
                geom.center,
                *radius,
                geom.start_angle_from_center.to_degrees(),
                angle.value(),
                start.color,
                start.pen_width,
                *steps,
                *direction,
            )
            .ok()?;

            Some(DrawCommand::Mesh { data: mesh_data })
        }

        // `produces_drawing()` guards entry — this arm is only reachable if
        // `produces_drawing` and the match above diverge, which would be a bug.
        _ => None,
    }
}

/// Push an [`SvgRecord`] for a completed line or arc drawing command.
///
/// Only compiled when the `svg` feature is enabled.
/// Must be called at the same call sites as `tessellate_command` so that
/// `svg_log` stays in sync with `commands`.
#[cfg(feature = "svg")]
pub(crate) fn push_svg_for_draw(
    command: &TurtleCommand,
    start: &TurtleParams,
    end_position: Vec2,
    svg_log: &mut crate::state::SvgLog,
) {
    use crate::state::SvgRecord;
    match command {
        TurtleCommand::Move(_) | TurtleCommand::Goto(_) => {
            svg_log.push(SvgRecord::Line {
                start: start.position,
                end: end_position,
                color: start.color,
                pen_width: start.pen_width,
            });
        }
        TurtleCommand::Circle {
            radius,
            angle,
            direction,
            ..
        } => {
            svg_log.push(SvgRecord::Arc {
                start_position: start.position,
                start_heading: start.heading,
                radius: *radius,
                angle: *angle,
                direction: *direction,
                color: start.color,
                pen_width: start.pen_width,
            });
        }
        _ => {}
    }
}

/// Execute a single turtle command, updating state and adding draw commands.
#[tracing::instrument(skip(state))]
pub(crate) fn execute_command(command: &TurtleCommand, state: &mut Turtle) {
    // Phase 1: side effects (fills, pen contours, reset, text).
    // Returns true if the command is fully handled — no params update or tessellation needed.
    if execute_command_side_effects(
        command,
        state.turtle_id,
        &mut state.params,
        &mut state.filling,
        &mut state.commands,
        &mut state.svg_log,
    ) {
        return;
    }

    // Phase 2: update TurtleParams (position, heading, colour, speed, etc.)
    let start_params = state.params.clone();
    command.apply_to_params(&mut state.params);

    // Phase 3: record fill vertices after movement (must follow params update)
    record_fill_vertices_after_movement(
        command,
        &start_params,
        state.turtle_id,
        &state.params,
        &mut state.filling,
    );

    // Phase 4: tessellate, push SVG record, and persist the committed drawing
    if let Some(draw_cmd) = tessellate_command(command, &start_params, state.params.position) {
        #[cfg(feature = "svg")]
        push_svg_for_draw(
            command,
            &start_params,
            state.params.position,
            &mut state.svg_log,
        );
        state.commands.push(draw_cmd);
    }
}

/// Execute command on a specific turtle by ID.
///
/// There is no ownership conflict here: `execute_command` only needs `&mut Turtle`
/// and never touches `TurtleWorld`, so we can obtain the mutable reference directly
/// from `get_turtle_mut` without any intermediate clone.
pub(crate) fn execute_command_with_id(
    command: &TurtleCommand,
    turtle_id: usize,
    world: &mut TurtleWorld,
) {
    if let Some(turtle) = world.get_turtle_mut(turtle_id) {
        execute_command(command, turtle);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::TurtleCommand;
    use crate::general::Degrees;
    use crate::shapes::TurtleShape;
    use crate::tweening::TweenController;

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
            svg_log: crate::state::SvgLog::default(),
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
        execute_command(&TurtleCommand::Turn(Degrees::new(-90.0)), &mut state);
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
