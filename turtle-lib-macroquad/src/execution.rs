//! Command execution logic

use crate::circle_geometry::{CircleDirection, CircleGeometry};
use crate::commands::TurtleCommand;
use crate::state::{DrawCommand, TurtleState, TurtleWorld};
use macroquad::prelude::*;

/// Execute a single turtle command, updating state and adding draw commands
pub fn execute_command(command: &TurtleCommand, state: &mut TurtleState, world: &mut TurtleWorld) {
    match command {
        TurtleCommand::Move(distance) => {
            let start = state.position;
            let dx = distance * state.heading.cos();
            let dy = distance * state.heading.sin();
            state.position = vec2(state.position.x + dx, state.position.y + dy);

            if state.pen_down {
                world.add_command(DrawCommand::Line {
                    start,
                    end: state.position,
                    color: state.color,
                    width: state.pen_width,
                });
                // Add circle at end point for smooth line joins
                world.add_command(DrawCommand::Circle {
                    center: state.position,
                    radius: state.pen_width / 2.0,
                    color: state.color,
                    filled: true,
                });
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

                world.add_command(DrawCommand::Arc {
                    center: geom.center,
                    radius: *radius - state.pen_width, // Adjust radius for pen width to keep arc inside
                    rotation: rotation_degrees,
                    arc: arc_degrees,
                    color: state.color,
                    width: state.pen_width,
                    sides: *steps as u8,
                });
            }

            // Update turtle position and heading
            state.position = geom.position_at_angle(angle.to_radians());
            state.heading = match direction {
                CircleDirection::Left => start_heading - angle.to_radians(),
                CircleDirection::Right => start_heading + angle.to_radians(),
            };
        }

        TurtleCommand::PenUp => {
            state.pen_down = false;
        }

        TurtleCommand::PenDown => {
            state.pen_down = true;
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

            if state.pen_down {
                world.add_command(DrawCommand::Line {
                    start,
                    end: state.position,
                    color: state.color,
                    width: state.pen_width,
                });
                // Add circle at end point for smooth line joins
                world.add_command(DrawCommand::Circle {
                    center: state.position,
                    radius: state.pen_width / 2.0,
                    color: state.color,
                    filled: true,
                });
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
    }
}

/// Execute all commands immediately (no animation)
pub fn execute_all_immediate(
    queue: &mut crate::commands::CommandQueue,
    state: &mut TurtleState,
    world: &mut TurtleWorld,
) {
    while let Some(command) = queue.next() {
        execute_command(command, state, world);
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
                world.add_command(DrawCommand::Line {
                    start: start_state.position,
                    end: end_state.position,
                    color: start_state.color,
                    width: start_state.pen_width,
                });
                // Add circle at end point for smooth line joins
                world.add_command(DrawCommand::Circle {
                    center: end_state.position,
                    radius: start_state.pen_width / 2.0,
                    color: start_state.color,
                    filled: true,
                });
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

                world.add_command(DrawCommand::Arc {
                    center: geom.center,
                    radius: *radius - start_state.pen_width / 2.0,
                    rotation: rotation_degrees,
                    arc: arc_degrees,
                    color: start_state.color,
                    width: start_state.pen_width,
                    sides: *steps as u8,
                });

                // Add endpoint circles for smooth joins
                world.add_command(DrawCommand::Circle {
                    center: start_state.position,
                    radius: start_state.pen_width / 2.0,
                    color: start_state.color,
                    filled: true,
                });
                world.add_command(DrawCommand::Circle {
                    center: end_state.position,
                    radius: start_state.pen_width / 2.0,
                    color: start_state.color,
                    filled: true,
                });
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
            speed: 100,
            visible: true,
            shape: TurtleShape::turtle(),
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
