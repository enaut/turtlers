//! Rendering logic using Macroquad

use crate::circle_geometry::{CircleDirection, CircleGeometry};
use crate::state::{DrawCommand, TurtleState, TurtleWorld};
use macroquad::prelude::*;

// Import the easing function from the tween crate
// To change the easing, change both this import and the usage in the draw_tween_arc_* functions below
// Available options: Linear, SineInOut, QuadInOut, CubicInOut, QuartInOut, QuintInOut,
//                    ExpoInOut, CircInOut, BackInOut, ElasticInOut, BounceInOut, etc.
// See https://easings.net/ for visual demonstrations
use tween::CubicInOut;

/// Render the entire turtle world
pub fn render_world(world: &TurtleWorld) {
    // Update camera zoom based on current screen size to prevent stretching
    let camera = Camera2D {
        zoom: vec2(1.0 / screen_width() * 2.0, 1.0 / screen_height() * 2.0),
        target: world.camera.target,
        ..Default::default()
    };

    // Set camera
    set_camera(&camera);

    // Draw all accumulated commands
    for cmd in &world.commands {
        match cmd {
            DrawCommand::Line {
                start,
                end,
                color,
                width,
            } => {
                draw_line(start.x, start.y, end.x, end.y, *width, *color);
            }
            DrawCommand::Circle {
                center,
                radius,
                color,
                filled,
            } => {
                if *filled {
                    draw_circle(center.x, center.y, *radius, *color);
                } else {
                    draw_circle_lines(center.x, center.y, *radius, 2.0, *color);
                }
            }
            DrawCommand::Arc {
                center,
                radius,
                rotation,
                arc,
                color,
                width,
                sides,
            } => {
                draw_arc(
                    center.x, center.y, *sides, *radius, *rotation, *width, *arc, *color,
                );
            }
            DrawCommand::FilledPolygon { vertices, color } => {
                if vertices.len() >= 3 {
                    draw_filled_polygon(vertices, *color);
                }
            }
        }
    }

    // Draw turtle if visible
    if world.turtle.visible {
        draw_turtle(&world.turtle);
    }

    // Reset to default camera
    set_default_camera();
}

/// Render the turtle world with active tween visualization
pub(crate) fn render_world_with_tween(
    world: &TurtleWorld,
    active_tween: Option<&crate::tweening::CommandTween>,
    zoom_level: f32,
) {
    // Update camera zoom based on current screen size to prevent stretching
    // Apply user zoom level by dividing by it (smaller zoom value = more zoomed in)
    let camera = Camera2D {
        zoom: vec2(
            1.0 / screen_width() * 2.0 / zoom_level,
            1.0 / screen_height() * 2.0 / zoom_level,
        ),
        target: world.camera.target,
        ..Default::default()
    };

    // Set camera
    set_camera(&camera);

    // Draw all accumulated commands
    for cmd in &world.commands {
        match cmd {
            DrawCommand::Line {
                start,
                end,
                color,
                width,
            } => {
                draw_line(start.x, start.y, end.x, end.y, *width, *color);
            }
            DrawCommand::Circle {
                center,
                radius,
                color,
                filled,
            } => {
                if *filled {
                    draw_circle(center.x, center.y, *radius, *color);
                } else {
                    draw_circle_lines(center.x, center.y, *radius, 2.0, *color);
                }
            }
            DrawCommand::Arc {
                center,
                radius,
                rotation,
                arc,
                color,
                width,
                sides,
            } => {
                draw_arc(
                    center.x, center.y, *sides, *radius, *rotation, *width, *arc, *color,
                );
            }
            DrawCommand::FilledPolygon { vertices, color } => {
                if vertices.len() >= 3 {
                    draw_filled_polygon(vertices, *color);
                }
            }
        }
    }

    // Draw in-progress tween line if pen is down
    if let Some(tween) = active_tween {
        if tween.start_state.pen_down {
            match &tween.command {
                crate::commands::TurtleCommand::Circle {
                    radius,
                    angle,
                    steps,
                    direction,
                } => {
                    // Draw arc segments from start to current position
                    draw_tween_arc(tween, *radius, *angle, *steps, *direction);
                }
                _ if should_draw_tween_line(&tween.command) => {
                    // Draw straight line for other movement commands
                    draw_line(
                        tween.start_state.position.x,
                        tween.start_state.position.y,
                        world.turtle.position.x,
                        world.turtle.position.y,
                        tween.start_state.pen_width,
                        tween.start_state.color,
                    );
                    // Add circle at current position for smooth line joins
                    draw_circle(
                        world.turtle.position.x,
                        world.turtle.position.y,
                        tween.start_state.pen_width / 2.0,
                        tween.start_state.color,
                    );
                }
                _ => {}
            }
        }
    }

    // Draw turtle if visible
    if world.turtle.visible {
        draw_turtle(&world.turtle);
    }

    // Reset to default camera
    set_default_camera();
}

fn should_draw_tween_line(command: &crate::commands::TurtleCommand) -> bool {
    use crate::commands::TurtleCommand;
    matches!(command, TurtleCommand::Move(..) | TurtleCommand::Goto(..))
}

/// Draw arc segments for circle tween animation
fn draw_tween_arc(
    tween: &crate::tweening::CommandTween,
    radius: f32,
    total_angle: f32,
    steps: usize,
    direction: CircleDirection,
) {
    let geom = CircleGeometry::new(
        tween.start_state.position,
        tween.start_state.heading,
        radius,
        direction,
    );

    // Debug: draw center
    draw_circle(geom.center.x, geom.center.y, 5.0, GRAY);

    // Calculate how much of the arc we've traveled based on tween progress
    // Use the same eased progress as the turtle position for synchronized animation
    let elapsed = (get_time() - tween.start_time) as f32;
    let t = (elapsed / tween.duration as f32).min(1.0);
    let progress = CubicInOut.tween(1.0, t); // tween from 0 to 1
    let angle_traveled = total_angle.to_radians() * progress;
    let (rotation_degrees, arc_degrees) = geom.draw_arc_params_partial(angle_traveled);

    // Adjust radius inward by half the line width so the line sits on the turtle's path
    let draw_radius = radius - tween.start_state.pen_width / 2.0;

    // Draw the partial arc
    draw_arc(
        geom.center.x,
        geom.center.y,
        steps as u8,
        draw_radius,
        rotation_degrees,
        tween.start_state.pen_width,
        arc_degrees,
        tween.start_state.color,
    );
}

/// Draw the turtle shape
pub fn draw_turtle(turtle: &TurtleState) {
    let rotated_vertices = turtle.shape.rotated_vertices(turtle.heading);

    if turtle.shape.filled {
        // Draw filled polygon (now supports concave shapes via ear clipping)
        if rotated_vertices.len() >= 3 {
            let absolute_vertices: Vec<Vec2> = rotated_vertices
                .iter()
                .map(|v| turtle.position + *v)
                .collect();

            draw_filled_polygon(&absolute_vertices, Color::new(0.0, 0.5, 1.0, 1.0));
        }
    } else {
        // Draw outline
        if !rotated_vertices.is_empty() {
            for i in 0..rotated_vertices.len() {
                let next_i = (i + 1) % rotated_vertices.len();
                let p1 = turtle.position + rotated_vertices[i];
                let p2 = turtle.position + rotated_vertices[next_i];
                draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, Color::new(0.0, 0.5, 1.0, 1.0));
            }
        }
    }
}

/// Draw a filled polygon using triangulation
fn draw_filled_polygon(vertices: &[Vec2], color: Color) {
    if vertices.len() < 3 {
        return;
    }

    // Flatten vertices into the format expected by earcutr: [x0, y0, x1, y1, ...]
    let flattened: Vec<f64> = vertices
        .iter()
        .flat_map(|v| vec![v.x as f64, v.y as f64])
        .collect();

    // Triangulate using earcutr (no holes, 2 dimensions)
    match earcutr::earcut(&flattened, &[], 2) {
        Ok(indices) => {
            // Draw each triangle
            for triangle in indices.chunks(3) {
                if triangle.len() == 3 {
                    let v0 = vertices[triangle[0]];
                    let v1 = vertices[triangle[1]];
                    let v2 = vertices[triangle[2]];
                    draw_triangle(v0, v1, v2, color);
                }
            }
        }
        Err(_) => {
            // Fallback: if triangulation fails, try simple fan triangulation
            let first = vertices[0];
            for i in 1..vertices.len() - 1 {
                draw_triangle(first, vertices[i], vertices[i + 1], color);
            }
        }
    }
}
