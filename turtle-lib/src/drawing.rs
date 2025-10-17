//! Rendering logic using Macroquad and Lyon tessellation

use crate::circle_geometry::{CircleDirection, CircleGeometry};
use crate::state::{DrawCommand, TurtleParams, TurtleWorld};
use crate::tessellation;
use macroquad::prelude::*;

// Import the easing function from the tween crate
// To change the easing, change both this import and the usage in the draw_tween_arc function below
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

    // Draw all accumulated commands from all turtles
    for turtle in &world.turtles {
        for cmd in &turtle.commands {
            match cmd {
                DrawCommand::Mesh { data } => {
                    draw_mesh(&data.to_mesh());
                }
            }
        }
    }

    // Draw all visible turtles
    for turtle in &world.turtles {
        if turtle.params.visible {
            draw_turtle(&turtle.params);
        }
    }

    // Reset to default camera
    set_default_camera();
}

/// Render the turtle world with active tween visualization
#[allow(clippy::too_many_lines)]
pub fn render_world_with_tweens(world: &TurtleWorld, zoom_level: f32) {
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

    // Draw all accumulated commands from all turtles
    for turtle in &world.turtles {
        for cmd in &turtle.commands {
            match cmd {
                DrawCommand::Mesh { data } => {
                    draw_mesh(&data.to_mesh());
                }
            }
        }
    }

    // Draw in-progress tween lines for all active tweens
    for turtle in world.turtles.iter() {
        if let Some(tween) = turtle.tween_controller.current_tween() {
            // Only draw if pen is down
            if tween.start_params.pen_down {
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
                        // Draw straight line for other movement commands (use tween's current position)
                        draw_line(
                            tween.start_params.position.x,
                            tween.start_params.position.y,
                            tween.current_position.x,
                            tween.current_position.y,
                            tween.start_params.pen_width,
                            tween.start_params.color,
                        );
                        // Add circle at current position for smooth line joins
                        draw_circle(
                            tween.current_position.x,
                            tween.current_position.y,
                            tween.start_params.pen_width / 2.0,
                            tween.start_params.color,
                        );
                    }
                    _ => {}
                }
            }
        }
    }

    // Draw live fill preview for all turtles that are currently filling
    for turtle in world.turtles.iter() {
        if let Some(ref fill_state) = turtle.filling {
            // Build all contours: completed contours + current contour with animation
            let mut all_contours: Vec<Vec<Vec2>> = Vec::new();

            // Add all completed contours
            for completed_contour in &fill_state.contours {
                let contour_vec2: Vec<Vec2> = completed_contour
                    .iter()
                    .map(|c| Vec2::new(c.x, c.y))
                    .collect();
                all_contours.push(contour_vec2);
            }

            // Build current contour with animation
            // Find the tween for this specific turtle
            let turtle_tween = turtle.tween_controller.current_tween();

            let mut current_preview: Vec<Vec2>;

            // If we have an active tween for this turtle, build the preview from tween state
            if let Some(tween) = turtle_tween {
                // Start with the existing contour vertices (vertices before the current tween)
                current_preview = fill_state
                    .current_contour
                    .iter()
                    .map(|c| Vec2::new(c.x, c.y))
                    .collect();

                // If we're animating a circle command with pen down, add progressive arc vertices
                if tween.start_params.pen_down {
                    if let crate::commands::TurtleCommand::Circle {
                        radius,
                        angle,
                        steps,
                        direction,
                    } = &tween.command
                    {
                        // Calculate partial arc vertices based on current progress
                        use crate::circle_geometry::CircleGeometry;
                        let geom = CircleGeometry::new(
                            tween.start_params.position,
                            tween.start_params.heading,
                            *radius,
                            *direction,
                        ); // Calculate progress
                        let elapsed = get_time() - tween.start_time;
                        let progress = (elapsed / tween.duration).min(1.0);
                        let eased_progress = CubicInOut.tween(1.0, progress as f32);

                        // Generate arc vertices for the partial arc
                        let num_samples = *steps.max(&1);
                        let samples_to_draw =
                            ((num_samples as f32 * eased_progress) as usize).max(1);

                        for i in 1..=samples_to_draw {
                            let sample_progress = i as f32 / num_samples as f32;
                            let current_angle = match direction {
                                crate::circle_geometry::CircleDirection::Left => {
                                    geom.start_angle_from_center
                                        - angle.to_radians() * sample_progress
                                }
                                crate::circle_geometry::CircleDirection::Right => {
                                    geom.start_angle_from_center
                                        + angle.to_radians() * sample_progress
                                }
                            };

                            let vertex = Vec2::new(
                                geom.center.x + radius * current_angle.cos(),
                                geom.center.y + radius * current_angle.sin(),
                            );
                            current_preview.push(vertex);
                        }
                    } else if matches!(
                        &tween.command,
                        crate::commands::TurtleCommand::Move(_)
                            | crate::commands::TurtleCommand::Goto(_)
                    ) {
                        // For Move/Goto commands, just add the current position from tween
                        current_preview.push(Vec2::new(
                            tween.current_position.x,
                            tween.current_position.y,
                        ));
                    }
                } else if matches!(
                    &tween.command,
                    crate::commands::TurtleCommand::Move(_)
                        | crate::commands::TurtleCommand::Goto(_)
                ) {
                    // For Move/Goto with pen up during filling, still add current position for preview
                    current_preview.push(Vec2::new(
                        tween.current_position.x,
                        tween.current_position.y,
                    ));
                }

                // Add current turtle position if not already included
                if let Some(last) = current_preview.last() {
                    let current_pos = tween.current_position;
                    // Use a larger threshold to reduce flickering from tiny movements
                    if (last.x - current_pos.x).abs() > 0.1 || (last.y - current_pos.y).abs() > 0.1
                    {
                        current_preview.push(Vec2::new(current_pos.x, current_pos.y));
                    }
                } else if !current_preview.is_empty() {
                    current_preview.push(Vec2::new(
                        tween.current_position.x,
                        tween.current_position.y,
                    ));
                }
            } else {
                // No active tween - use the actual current contour from fill state
                current_preview = fill_state
                    .current_contour
                    .iter()
                    .map(|c| Vec2::new(c.x, c.y))
                    .collect();

                // No active tween - just show current state
                if !current_preview.is_empty() {
                    if let Some(last) = current_preview.last() {
                        let current_pos = turtle.params.position;
                        if (last.x - current_pos.x).abs() > 0.1
                            || (last.y - current_pos.y).abs() > 0.1
                        {
                            current_preview.push(Vec2::new(current_pos.x, current_pos.y));
                        }
                    }
                }
            }

            // Add current contour to all contours if it has enough vertices
            if current_preview.len() >= 3 {
                all_contours.push(current_preview);
            }

            // Tessellate and draw all contours together using multi-contour tessellation
            if !all_contours.is_empty() {
                match crate::tessellation::tessellate_multi_contour(
                    &all_contours,
                    fill_state.fill_color,
                ) {
                    Ok(mesh_data) => {
                        draw_mesh(&mesh_data.to_mesh());
                    }
                    Err(e) => {
                        tracing::error!("Failed to tessellate fill preview: {:?}", e);
                    }
                }
            }
        }
    }

    // Draw all visible turtles
    for turtle in &world.turtles {
        if turtle.params.visible {
            draw_turtle(&turtle.params);
        }
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
        tween.start_params.position,
        tween.start_params.heading,
        radius,
        direction,
    );

    // Debug: draw center using Lyon tessellation
    if let Ok(mesh_data) = crate::tessellation::tessellate_circle(geom.center, 5.0, GRAY, true, 1.0)
    {
        draw_mesh(&mesh_data.to_mesh());
    }

    // Calculate how much of the arc we've traveled based on tween progress
    // Use the same eased progress as the turtle position for synchronized animation
    let elapsed = get_time() - tween.start_time;
    let t = (elapsed / tween.duration).min(1.0);
    let progress = CubicInOut.tween(1.0, t as f32); // tween from 0 to 1

    // Use Lyon to tessellate and draw the partial arc
    if let Ok(mesh_data) = crate::tessellation::tessellate_arc(
        geom.center,
        radius,
        geom.start_angle_from_center.to_degrees(),
        total_angle * progress,
        tween.start_params.color,
        tween.start_params.pen_width,
        ((steps as f32 * progress).ceil() as usize).max(1),
        direction,
    ) {
        draw_mesh(&mesh_data.to_mesh());
    }
}

/// Draw the turtle shape
pub fn draw_turtle(turtle_params: &TurtleParams) {
    let rotated_vertices = turtle_params.shape.rotated_vertices(turtle_params.heading);

    if turtle_params.shape.filled {
        // Draw filled polygon using Lyon tessellation
        if rotated_vertices.len() >= 3 {
            let absolute_vertices: Vec<Vec2> = rotated_vertices
                .iter()
                .map(|v| turtle_params.position + *v)
                .collect();

            // Use Lyon for turtle shape too
            if let Ok(mesh_data) =
                tessellation::tessellate_polygon(&absolute_vertices, Color::new(0.0, 0.5, 1.0, 1.0))
            {
                draw_mesh(&mesh_data.to_mesh());
            } else {
                // Fallback to simple triangle fan if Lyon fails
                let first = absolute_vertices[0];
                for i in 1..absolute_vertices.len() - 1 {
                    draw_triangle(
                        first,
                        absolute_vertices[i],
                        absolute_vertices[i + 1],
                        Color::new(0.0, 0.5, 1.0, 1.0),
                    );
                }
            }
        }
    } else {
        // Draw outline
        if !rotated_vertices.is_empty() {
            for i in 0..rotated_vertices.len() {
                let next_i = (i + 1) % rotated_vertices.len();
                let p1 = turtle_params.position + rotated_vertices[i];
                let p2 = turtle_params.position + rotated_vertices[next_i];
                draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, Color::new(0.0, 0.5, 1.0, 1.0));
            }
        }
    }
}
