//! Centralised behavioural contract for `TurtleCommand`.
//!
//! All knowledge about what a command does to `TurtleParams`, how long it
//! animates, and whether it produces a drawable stroke lives here.
//!
//! Adding a new `TurtleCommand` variant requires editing this file (and
//! `execute_command_side_effects` / `tessellate_command` in `execution.rs`
//! if the variant has side effects or produces a mesh).

use crate::circle_geometry::{CircleDirection, CircleGeometry};
use crate::commands::TurtleCommand;
use crate::general::{AnimationSpeed, Radians};
use crate::state::TurtleParams;
use crate::tweening::normalize_angle;
use macroquad::prelude::vec2;

impl TurtleCommand {
    /// Apply this command's effect to `params` in place.
    ///
    /// This is the **single source of truth** for what a command changes in
    /// `TurtleParams`. Used by:
    /// - `execute_command()` — instant-mode path, after side-effects return `false`
    /// - `TweenController::calculate_target_state()` — animated-mode target computation
    ///
    /// Variants handled by `execute_command_side_effects` (`BeginFill`, `EndFill`,
    /// `PenUp`, `PenDown`, `WriteText`, `Reset`) are included here so that
    /// `calculate_target_state` can produce a correct tween target.  In the
    /// `execute_command()` call path those variants never reach this method because
    /// `execute_command_side_effects` returns `true` and the caller returns early —
    /// there is no double-application.
    pub(crate) fn apply_to_params(&self, params: &mut TurtleParams) {
        match self {
            TurtleCommand::Move(dist) => {
                let dx = dist.value() * params.heading.cos();
                let dy = dist.value() * params.heading.sin();
                params.position = vec2(params.position.x + dx, params.position.y + dy);
            }
            TurtleCommand::Turn(angle) => {
                params.heading = normalize_angle(params.heading + angle.as_radians().value());
            }
            TurtleCommand::Circle {
                radius,
                angle,
                direction,
                ..
            } => {
                let geom = CircleGeometry::new(
                    params.position,
                    Radians::new(params.heading),
                    radius.value(),
                    *direction,
                );
                let angle_rad = angle.as_radians().value();
                params.position = geom.position_at_angle(angle_rad);
                params.heading = normalize_angle(match direction {
                    CircleDirection::Left => params.heading - angle_rad,
                    CircleDirection::Right => params.heading + angle_rad,
                });
            }
            TurtleCommand::Goto(coord) => {
                // Y-flip: turtle graphics Y+ = up; Macroquad Y+ = down
                params.position = vec2(coord.x, -coord.y);
            }
            TurtleCommand::SetHeading(heading) => {
                params.heading = normalize_angle(-heading.as_radians().value());
            }
            TurtleCommand::SetColor(color) => {
                params.color = *color;
            }
            TurtleCommand::SetFillColor(color) => {
                params.fill_color = *color;
            }
            TurtleCommand::SetPenWidth(width) => {
                params.pen_width = *width;
            }
            TurtleCommand::SetSpeed(speed) => {
                params.speed = *speed;
            }
            TurtleCommand::SetShape(shape) => {
                params.shape = shape.clone();
            }
            TurtleCommand::PenUp => {
                params.pen_down = false;
            }
            TurtleCommand::PenDown => {
                params.pen_down = true;
            }
            TurtleCommand::ShowTurtle => {
                params.visible = true;
            }
            TurtleCommand::HideTurtle => {
                params.visible = false;
            }
            TurtleCommand::Reset => {
                *params = TurtleParams::default();
            }
            // Fill/text commands do not change TurtleParams for tweening purposes;
            // their effects are handled entirely by execute_command_side_effects.
            TurtleCommand::BeginFill | TurtleCommand::EndFill | TurtleCommand::WriteText { .. } => {
            }
        }
    }

    /// Duration in seconds for this command's animation at the given speed.
    ///
    /// Returns `0.01` (minimum) for commands that have no animated component.
    /// This is the **single source of truth**; replaces
    /// `TweenController::calculate_duration_with_state` in `tweening.rs`.
    pub(crate) fn animation_duration(&self, params: &TurtleParams, speed: AnimationSpeed) -> f64 {
        let AnimationSpeed::Animated(mut spd) = speed else {
            // Instant mode — duration is irrelevant; return the minimum so tweener
            // infrastructure still has a valid duration if called accidentally.
            return f64::from(0.01_f32);
        };

        // Exponential speed scaling for high values (matches original behaviour)
        if spd > 100.0 {
            spd *= spd / 100.0;
        }

        let base: f32 = match self {
            TurtleCommand::Move(dist) => dist.value().abs() / spd,
            TurtleCommand::Turn(angle) => angle.value().abs() / (spd * 1.8),
            TurtleCommand::Circle { radius, angle, .. } => {
                let arc_length = radius.value() * angle.as_radians().value().abs();
                arc_length / spd
            }
            TurtleCommand::Goto(target) => {
                let screen_target = vec2(target.x, -target.y);
                let dx = screen_target.x - params.position.x;
                let dy = screen_target.y - params.position.y;
                (dx * dx + dy * dy).sqrt() / spd
            }
            _ => 0.0,
        };

        f64::from(base.max(0.01))
    }

    /// Whether executing this command (when pen is down) produces a stroke or fill mesh.
    ///
    /// This is the **single source of truth**; replaces
    /// `TweenController::command_creates_drawing` in `tweening.rs`.
    #[must_use]
    pub(crate) fn produces_drawing(&self) -> bool {
        matches!(
            self,
            TurtleCommand::Move(_) | TurtleCommand::Circle { .. } | TurtleCommand::Goto(_)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::general::{AnimationSpeed, Color, Coordinate, Degrees};
    use crate::shapes::TurtleShape;

    fn make_test_params() -> TurtleParams {
        TurtleParams {
            position: vec2(0.0, 0.0),
            heading: 0.0,
            pen_down: true,
            pen_width: 1.0,
            color: Color::new(0.0, 0.0, 0.0, 1.0),
            fill_color: None,
            visible: true,
            shape: TurtleShape::turtle(),
            speed: AnimationSpeed::Animated(100.0),
        }
    }

    #[test]
    fn test_goto_duration_cartesian_inversion() {
        let mut params = make_test_params();
        // Set screen position to (0, 100), which corresponds to Cartesian (0, -100)
        params.position = vec2(0.0, 100.0);

        // Move to Cartesian (0, 100), which in screen space is (0, -100)
        // Distance should be 200 pixels!
        let cmd = TurtleCommand::Goto(Coordinate::new(0.0, 100.0));
        let duration = cmd.animation_duration(&params, AnimationSpeed::Animated(100.0));

        // At 100 px/sec across 200 pixels, duration should be 2.0 seconds
        assert!(
            (duration - 2.0).abs() < 0.01,
            "Expected duration ~2.0s for 200px move, got {duration}"
        );
    }

    #[test]
    fn test_set_heading_degrees_and_instant_duration() {
        let mut params = make_test_params();

        // 90° = North (in screen coordinates: -pi/2)
        let cmd_north = TurtleCommand::SetHeading(Degrees::new(90.0));
        cmd_north.apply_to_params(&mut params);
        let expected_north = -std::f32::consts::FRAC_PI_2;
        assert!(
            (params.heading - expected_north).abs() < 0.001,
            "Heading 90° should be North (-π/2), got {}",
            params.heading
        );

        // SetHeading should be instant (0.01 minimum duration)
        let duration = cmd_north.animation_duration(&params, AnimationSpeed::Animated(100.0));
        assert!((duration - 0.01).abs() < 0.001);

        // 0° = East (0 radians)
        let cmd_east = TurtleCommand::SetHeading(Degrees::new(0.0));
        cmd_east.apply_to_params(&mut params);
        assert!((params.heading - 0.0).abs() < 0.001);

        // 270° = South (+pi/2 radians in screen coords)
        let cmd_south = TurtleCommand::SetHeading(Degrees::new(270.0));
        cmd_south.apply_to_params(&mut params);
        let expected_south = std::f32::consts::FRAC_PI_2;
        assert!(
            (params.heading - expected_south).abs() < 0.001,
            "Heading 270° should be South (π/2), got {}",
            params.heading
        );
    }
}

