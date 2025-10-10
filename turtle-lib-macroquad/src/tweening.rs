//! Tweening system for smooth animations

use crate::circle_geometry::{CircleDirection, CircleGeometry};
use crate::commands::{CommandQueue, TurtleCommand};
use crate::general::AnimationSpeed;
use crate::state::TurtleState;
use macroquad::prelude::*;
use tween::{CubicInOut, TweenValue, Tweener};

// Newtype wrapper for Vec2 to implement TweenValue
#[derive(Debug, Clone, Copy)]
pub(crate) struct TweenVec2(Vec2);

impl TweenValue for TweenVec2 {
    fn scale(self, scalar: f32) -> Self {
        TweenVec2(self.0 * scalar)
    }
}

impl std::ops::Add for TweenVec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        TweenVec2(self.0 + rhs.0)
    }
}

impl std::ops::Sub for TweenVec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        TweenVec2(self.0 - rhs.0)
    }
}

impl From<Vec2> for TweenVec2 {
    fn from(v: Vec2) -> Self {
        TweenVec2(v)
    }
}

impl From<TweenVec2> for Vec2 {
    fn from(v: TweenVec2) -> Self {
        v.0
    }
}

/// Controls tweening of turtle commands
pub struct TweenController {
    queue: CommandQueue,
    current_tween: Option<CommandTween>,
    speed: AnimationSpeed,
}

pub(crate) struct CommandTween {
    pub command: TurtleCommand,
    pub start_time: f64,
    pub duration: f64,
    pub start_state: TurtleState,
    pub target_state: TurtleState,
    pub position_tweener: Tweener<TweenVec2, f32, CubicInOut>,
    pub heading_tweener: Tweener<f32, f32, CubicInOut>,
    pub pen_width_tweener: Tweener<f32, f32, CubicInOut>,
}

impl TweenController {
    pub fn new(queue: CommandQueue, speed: AnimationSpeed) -> Self {
        Self {
            queue,
            current_tween: None,
            speed,
        }
    }

    pub fn set_speed(&mut self, speed: AnimationSpeed) {
        self.speed = speed;
    }

    /// Update the tween, returns Vec of (command, start_state, end_state) for all completed commands this frame
    /// Each command has its own start_state and end_state pair
    pub fn update(
        &mut self,
        state: &mut TurtleState,
    ) -> Vec<(TurtleCommand, TurtleState, TurtleState)> {
        // In instant mode, execute commands up to the draw calls per frame limit
        if let AnimationSpeed::Instant(max_draw_calls) = self.speed {
            let mut completed_commands = Vec::new();
            let mut draw_call_count = 0;

            loop {
                let command = match self.queue.next() {
                    Some(cmd) => cmd.clone(),
                    None => break,
                };

                // Capture start state BEFORE executing this command
                let start_state = state.clone();

                // Handle SetSpeed command to potentially switch modes
                if let TurtleCommand::SetSpeed(new_speed) = &command {
                    state.set_speed(*new_speed);
                    self.speed = *new_speed;
                    // If speed switched to animated mode, exit instant mode processing
                    if matches!(self.speed, AnimationSpeed::Animated(_)) {
                        break;
                    }
                    continue;
                }

                // Execute command immediately
                let target_state = self.calculate_target_state(state, &command);
                *state = target_state.clone();

                // Capture end state AFTER executing this command
                let end_state = state.clone();

                // Collect drawable commands with their individual start and end states
                if Self::command_creates_drawing(&command) {
                    completed_commands.push((command, start_state, end_state));
                    draw_call_count += 1;

                    // Stop if we've reached the draw call limit for this frame
                    if draw_call_count >= max_draw_calls {
                        break;
                    }
                }
            }

            return completed_commands;
        }

        // Process current tween
        if let Some(ref mut tween) = self.current_tween {
            let elapsed = (get_time() - tween.start_time) as f32;

            // Use tweeners to calculate current values
            // For circles, calculate position along the arc instead of straight line
            let progress = tween.heading_tweener.move_to(elapsed);

            state.position = match &tween.command {
                TurtleCommand::Circle {
                    radius,
                    angle,
                    direction,
                    ..
                } => {
                    let angle_traveled = angle.to_radians() * progress;
                    calculate_circle_position(
                        tween.start_state.position,
                        tween.start_state.heading,
                        *radius,
                        angle_traveled,
                        *direction,
                    )
                }
                _ => {
                    // For non-circle commands, use normal position tweening
                    tween.position_tweener.move_to(elapsed).into()
                }
            };

            // Heading changes proportionally with progress for all commands
            state.heading = normalize_angle(match &tween.command {
                TurtleCommand::Circle {
                    angle, direction, ..
                } => match direction {
                    CircleDirection::Left => {
                        tween.start_state.heading - angle.to_radians() * progress
                    }
                    CircleDirection::Right => {
                        tween.start_state.heading + angle.to_radians() * progress
                    }
                },
                TurtleCommand::Turn(angle) => {
                    tween.start_state.heading + angle.to_radians() * progress
                }
                TurtleCommand::SetHeading(_) | _ => {
                    // For other commands that change heading, lerp directly
                    let heading_diff = tween.target_state.heading - tween.start_state.heading;
                    tween.start_state.heading + heading_diff * progress
                }
            });
            state.pen_width = tween.pen_width_tweener.move_to(elapsed);

            // Discrete properties (switch at 50% progress)
            let progress = (elapsed / tween.duration as f32).min(1.0);
            if progress >= 0.5 {
                state.pen_down = tween.target_state.pen_down;
                state.color = tween.target_state.color;
                state.fill_color = tween.target_state.fill_color;
                state.visible = tween.target_state.visible;
                state.shape = tween.target_state.shape.clone();
            }

            // Check if tween is finished (use heading_tweener as it's used by all commands)
            if tween.heading_tweener.is_finished() {
                // Tween complete, finalize state
                let start_state = tween.start_state.clone();
                *state = tween.target_state.clone();
                let end_state = state.clone();

                // Return the completed command and start/end states to add draw commands
                let completed_command = tween.command.clone();
                self.current_tween = None;

                // Only return command if it creates drawable elements
                if Self::command_creates_drawing(&completed_command) {
                    return vec![(completed_command, start_state, end_state)];
                }
            }

            return Vec::new();
        }

        // Start next tween
        if let Some(command) = self.queue.next() {
            let command_clone = command.clone();

            // Handle SetSpeed command specially
            if let TurtleCommand::SetSpeed(new_speed) = &command_clone {
                state.set_speed(*new_speed);
                self.speed = *new_speed;
                // If switched to instant mode, process commands immediately
                if matches!(self.speed, AnimationSpeed::Instant(_)) {
                    return self.update(state); // Recursively process in instant mode
                }
                // For animated mode speed changes, continue to next command
                return self.update(state);
            }

            let speed = state.speed; // Extract speed before borrowing self
            let duration = self.calculate_duration(&command_clone, speed);

            // Calculate target state
            let target_state = self.calculate_target_state(state, &command_clone);

            // Create tweeners for smooth animation
            let position_tweener = Tweener::new(
                TweenVec2::from(state.position),
                TweenVec2::from(target_state.position),
                duration as f32,
                CubicInOut,
            );

            let heading_tweener = Tweener::new(
                0.0, // We'll handle angle wrapping separately
                1.0,
                duration as f32,
                CubicInOut,
            );

            let pen_width_tweener = Tweener::new(
                state.pen_width,
                target_state.pen_width,
                duration as f32,
                CubicInOut,
            );

            self.current_tween = Some(CommandTween {
                command: command_clone,
                start_time: get_time(),
                duration,
                start_state: state.clone(),
                target_state,
                position_tweener,
                heading_tweener,
                pen_width_tweener,
            });
        }

        Vec::new()
    }

    pub fn is_complete(&self) -> bool {
        self.current_tween.is_none() && self.queue.is_complete()
    }

    /// Get the current active tween if one is in progress
    pub(crate) fn current_tween(&self) -> Option<&CommandTween> {
        self.current_tween.as_ref()
    }

    fn command_creates_drawing(command: &TurtleCommand) -> bool {
        matches!(
            command,
            TurtleCommand::Move(_) | TurtleCommand::Circle { .. } | TurtleCommand::Goto(_)
        )
    }

    fn calculate_duration(&self, command: &TurtleCommand, speed: AnimationSpeed) -> f64 {
        let speed = speed.value();

        let base_time = match command {
            TurtleCommand::Move(dist) => dist.abs() / speed,
            TurtleCommand::Turn(angle) => {
                // Rotation speed: assume 180 degrees per second at speed 100
                angle.abs() / (speed * 1.8)
            }
            TurtleCommand::Circle { radius, angle, .. } => {
                let arc_length = radius * angle.to_radians().abs();
                arc_length / speed
            }
            TurtleCommand::Goto(_target) => {
                // Calculate distance (handled in calculate_target_state)
                0.1 // Placeholder, will be calculated properly
            }
            _ => 0.0, // Instant commands
        };
        base_time.max(0.01) as f64 // Minimum duration
    }

    fn calculate_target_state(
        &self,
        current: &TurtleState,
        command: &TurtleCommand,
    ) -> TurtleState {
        let mut target = current.clone();

        match command {
            TurtleCommand::Move(dist) => {
                let dx = dist * current.heading.cos();
                let dy = dist * current.heading.sin();
                target.position = vec2(current.position.x + dx, current.position.y + dy);
            }
            TurtleCommand::Turn(angle) => {
                target.heading = normalize_angle(current.heading + angle.to_radians());
            }
            TurtleCommand::Circle {
                radius,
                angle,
                direction,
                ..
            } => {
                // Use helper function to calculate final position
                target.position = calculate_circle_position(
                    current.position,
                    current.heading,
                    *radius,
                    angle.to_radians(),
                    *direction,
                );
                target.heading = normalize_angle(match direction {
                    CircleDirection::Left => current.heading - angle.to_radians(),
                    CircleDirection::Right => current.heading + angle.to_radians(),
                });
            }
            TurtleCommand::Goto(coord) => {
                target.position = *coord;
            }
            TurtleCommand::SetHeading(heading) => {
                target.heading = normalize_angle(*heading);
            }
            TurtleCommand::SetColor(color) => {
                target.color = *color;
            }
            TurtleCommand::SetPenWidth(width) => {
                target.pen_width = *width;
            }
            TurtleCommand::SetSpeed(speed) => {
                target.speed = *speed;
            }
            TurtleCommand::SetShape(shape) => {
                target.shape = shape.clone();
            }
            TurtleCommand::PenUp => {
                target.pen_down = false;
            }
            TurtleCommand::PenDown => {
                target.pen_down = true;
            }
            TurtleCommand::ShowTurtle => {
                target.visible = true;
            }
            TurtleCommand::HideTurtle => {
                target.visible = false;
            }
            TurtleCommand::SetFillColor(color) => {
                target.fill_color = *color;
            }
        }

        target
    }
}

/// Calculate position on a circular arc
fn calculate_circle_position(
    start_pos: Vec2,
    start_heading: f32,
    radius: f32,
    angle_traveled: f32, // How much of the total angle we've traveled (in radians)
    direction: CircleDirection,
) -> Vec2 {
    let geom = CircleGeometry::new(start_pos, start_heading, radius, direction);
    geom.position_at_angle(angle_traveled)
}

/// Normalize angle to range [-PI, PI] to prevent floating-point drift
fn normalize_angle(angle: f32) -> f32 {
    let two_pi = std::f32::consts::PI * 2.0;
    let mut normalized = angle % two_pi;

    // Ensure result is in [-PI, PI]
    if normalized > std::f32::consts::PI {
        normalized -= two_pi;
    } else if normalized < -std::f32::consts::PI {
        normalized += two_pi;
    }

    normalized
}
