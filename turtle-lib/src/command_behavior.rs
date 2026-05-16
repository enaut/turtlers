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
use crate::general::AnimationSpeed;
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
                let dx = dist * params.heading.cos();
                let dy = dist * params.heading.sin();
                params.position = vec2(params.position.x + dx, params.position.y + dy);
            }
            TurtleCommand::Turn(angle) => {
                params.heading = normalize_angle(params.heading + angle.to_radians());
            }
            TurtleCommand::Circle {
                radius,
                angle,
                direction,
                ..
            } => {
                let geom =
                    CircleGeometry::new(params.position, params.heading, *radius, *direction);
                params.position = geom.position_at_angle(angle.to_radians());
                params.heading = normalize_angle(match direction {
                    CircleDirection::Left => params.heading - angle.to_radians(),
                    CircleDirection::Right => params.heading + angle.to_radians(),
                });
            }
            TurtleCommand::Goto(coord) => {
                // Y-flip: turtle graphics Y+ = up; Macroquad Y+ = down
                params.position = vec2(coord.x, -coord.y);
            }
            TurtleCommand::SetHeading(heading) => {
                params.heading = normalize_angle(*heading);
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
            TurtleCommand::Move(dist) => dist.abs() / spd,
            TurtleCommand::Turn(angle) => angle.abs() / (spd * 1.8),
            TurtleCommand::Circle { radius, angle, .. } => {
                let arc_length = radius * angle.to_radians().abs();
                arc_length / spd
            }
            TurtleCommand::Goto(target) => {
                let dx = target.x - params.position.x;
                let dy = target.y - params.position.y;
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
