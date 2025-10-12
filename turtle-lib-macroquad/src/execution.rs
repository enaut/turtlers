//! Command execution logic

use crate::circle_geometry::{CircleDirection, CircleGeometry};
use crate::commands::TurtleCommand;
use crate::state::{DrawCommand, TurtleState, TurtleWorld};
use crate::tessellation;
use macroquad::prelude::*;

#[cfg(test)]
use crate::general::AnimationSpeed;

/// Execute a single turtle command, updating state and adding draw commands
pub fn execute_command(command: &TurtleCommand, state: &mut TurtleState, world: &mut TurtleWorld) {
    match command {
        TurtleCommand::Move(distance) => {
            let start = state.position;
            let dx = distance * state.heading.cos();
            let dy = distance * state.heading.sin();
            state.position = vec2(state.position.x + dx, state.position.y + dy);

            // Record vertex for fill if filling
            state.record_fill_vertex();

            if state.pen_down {
                // Draw line segment with round caps (caps handled by tessellate_stroke)
                if let Ok(mesh_data) = tessellation::tessellate_stroke(
                    &[start, state.position],
                    state.color,
                    state.pen_width,
                    false, // not closed
                ) {
                    world.add_command(DrawCommand::Mesh(mesh_data));
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
                let (rotation_degrees, arc_degrees) = geom.draw_arc_params(*angle);

                // Use Lyon to tessellate the arc
                if let Ok(mesh_data) = tessellation::tessellate_arc(
                    geom.center,
                    *radius,
                    rotation_degrees,
                    arc_degrees,
                    state.color,
                    state.pen_width,
                    *steps as u8,
                ) {
                    world.add_command(DrawCommand::Mesh(mesh_data));
                }
            }

            // Update turtle position and heading
            state.position = geom.position_at_angle(angle.to_radians());
            state.heading = match direction {
                CircleDirection::Left => start_heading - angle.to_radians(),
                CircleDirection::Right => start_heading + angle.to_radians(),
            };

            // Record vertices along arc for fill if filling
            state.record_fill_vertices_for_arc(
                geom.center,
                *radius,
                geom.start_angle_from_center,
                angle.to_radians(),
                *direction,
                *steps as u32,
            );
        }

        TurtleCommand::PenUp => {
            state.pen_down = false;
            // Close current contour if filling
            if state.filling.is_some() {
                eprintln!("PenUp: Closing current contour");
            }
            state.close_fill_contour();
        }

        TurtleCommand::PenDown => {
            state.pen_down = true;
            // Start new contour if filling
            if state.filling.is_some() {
                eprintln!(
                    "PenDown: Starting new contour at position ({}, {})",
                    state.position.x, state.position.y
                );
            }
            state.start_fill_contour();
        }

        TurtleCommand::SetColor(color) => {
            state.color = *color;
        }

        TurtleCommand::SetFillColor(color) => {
            state.fill_color = *color;
        }

        TurtleCommand::SetPenWidth(width) => {
            state.pen_width = *width;
        }

        TurtleCommand::SetSpeed(speed) => {
            state.set_speed(*speed);
        }

        TurtleCommand::SetShape(shape) => {
            state.shape = shape.clone();
        }

        TurtleCommand::Goto(coord) => {
            let start = state.position;
            state.position = *coord;

            // Record vertex for fill if filling
            state.record_fill_vertex();

            if state.pen_down {
                // Draw line segment with round caps
                if let Ok(mesh_data) = tessellation::tessellate_stroke(
                    &[start, state.position],
                    state.color,
                    state.pen_width,
                    false, // not closed
                ) {
                    world.add_command(DrawCommand::Mesh(mesh_data));
                }
            }
        }

        TurtleCommand::SetHeading(heading) => {
            state.heading = *heading;
        }

        TurtleCommand::ShowTurtle => {
            state.visible = true;
        }

        TurtleCommand::HideTurtle => {
            state.visible = false;
        }

        TurtleCommand::BeginFill => {
            if state.filling.is_some() {
                eprintln!("Warning: begin_fill() called while already filling");
            }

            let fill_color = state.fill_color.unwrap_or_else(|| {
                eprintln!("Warning: No fill_color set, using black");
                BLACK
            });

            state.begin_fill(fill_color);
        }

        TurtleCommand::EndFill => {
            if let Some(mut fill_state) = state.filling.take() {
                // Close final contour if it has vertices
                if !fill_state.current_contour.is_empty() {
                    fill_state.contours.push(fill_state.current_contour);
                }

                // Debug output
                eprintln!("=== EndFill Debug ===");
                eprintln!("Total contours: {}", fill_state.contours.len());
                for (i, contour) in fill_state.contours.iter().enumerate() {
                    eprintln!("  Contour {}: {} vertices", i, contour.len());
                }

                // Create fill command - Lyon will handle EvenOdd automatically with multiple contours
                if !fill_state.contours.is_empty() {
                    if let Ok(mesh_data) = tessellation::tessellate_multi_contour(
                        &fill_state.contours,
                        fill_state.fill_color,
                    ) {
                        eprintln!(
                            "Successfully tessellated {} contours",
                            fill_state.contours.len()
                        );
                        world.add_command(DrawCommand::Mesh(mesh_data));
                    } else {
                        eprintln!("ERROR: Failed to tessellate contours!");
                    }
                }
            } else {
                eprintln!("Warning: end_fill() called without begin_fill()");
            }
        }
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
                    false, // not closed
                ) {
                    world.add_command(DrawCommand::Mesh(mesh_data));
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
                let (rotation_degrees, arc_degrees) = geom.draw_arc_params(*angle);

                // Use Lyon to tessellate the arc
                if let Ok(mesh_data) = tessellation::tessellate_arc(
                    geom.center,
                    *radius,
                    rotation_degrees,
                    arc_degrees,
                    start_state.color,
                    start_state.pen_width,
                    *steps as u8,
                ) {
                    world.add_command(DrawCommand::Mesh(mesh_data));
                }
            }
        }
        TurtleCommand::BeginFill | TurtleCommand::EndFill => {
            // No immediate drawing for fill commands, handled in execute_command
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
            turtle: state.clone(),
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
