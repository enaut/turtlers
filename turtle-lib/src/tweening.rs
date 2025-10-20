//! Tweening system for smooth animations

use crate::circle_geometry::{CircleDirection, CircleGeometry};
use crate::commands::{CommandQueue, TurtleCommand};
use crate::general::AnimationSpeed;
use crate::state::{Turtle, TurtleParams};
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
#[derive(Clone, Debug, Default)]
pub struct TweenController {
    queue: CommandQueue,
    current_tween: Option<CommandTween>,
    speed: AnimationSpeed,
}

#[derive(Clone, Debug)]
pub struct CommandTween {
    pub turtle_id: usize,
    pub command: TurtleCommand,
    pub start_time: f64,
    pub duration: f64,
    pub start_params: TurtleParams,
    pub target_params: TurtleParams,
    pub current_position: Vec2,
    pub current_heading: f32,
    position_tweener: Tweener<TweenVec2, f64, CubicInOut>,
    heading_tweener: Tweener<f32, f64, CubicInOut>,
    pen_width_tweener: Tweener<f32, f64, CubicInOut>,
}

impl TweenController {
    #[must_use]
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

    /// Append commands to the queue
    pub fn append_commands(&mut self, new_queue: CommandQueue) {
        self.queue.extend(new_queue);
    }

    /// Update the tween, returns `Vec` of (`command`, `start_state`, `end_state`) for all completed commands this frame
    /// Also takes commands vec to handle side effects like fill operations
    /// Each `command` has its own `start_state` and `end_state` pair
    #[allow(clippy::too_many_lines)]
    pub fn update(state: &mut Turtle) -> Vec<(TurtleCommand, TurtleParams, TurtleParams)> {
        // In instant mode, execute commands up to the draw calls per frame limit
        if let AnimationSpeed::Instant(max_draw_calls) = state.tween_controller.speed {
            let mut completed_commands: Vec<(TurtleCommand, TurtleParams, TurtleParams)> =
                Vec::new();
            let mut draw_call_count = 0;

            // Consume commands from the real queue so the current_index advances
            while let Some(command) = state.tween_controller.queue.next() {
                // Handle SetSpeed command to potentially switch modes
                if let TurtleCommand::SetSpeed(new_speed) = &command {
                    state.params.speed = *new_speed;
                    state.tween_controller.speed = *new_speed;
                    if matches!(state.tween_controller.speed, AnimationSpeed::Animated(_)) {
                        break;
                    }
                    continue;
                }

                // Execute side-effect-only commands using centralized helper
                if crate::execution::execute_command_side_effects(&command, state) {
                    continue; // Command fully handled
                }

                // Save start state and compute target state
                let start_params = state.params.clone();
                let target_params = Self::calculate_target_state(&start_params, &command);

                // Update state to the target (instant execution)
                state.params = target_params.clone();

                // Record fill vertices AFTER movement
                crate::execution::record_fill_vertices_after_movement(
                    &command,
                    &start_params,
                    state,
                );

                // Collect drawable commands (return start and target so caller can create draw meshes)
                if Self::command_creates_drawing(&command) && start_params.pen_down {
                    completed_commands.push((command, start_params.clone(), target_params.clone()));
                    draw_call_count += 1;
                    if draw_call_count >= max_draw_calls {
                        break;
                    }
                }
            }

            return completed_commands;
        }

        // Process current tween
        if let Some(ref mut tween) = state.tween_controller.current_tween {
            let elapsed = get_time() - tween.start_time;

            // Use tweeners to calculate current values
            // For circles, calculate position along the arc instead of straight line
            let progress = tween.heading_tweener.move_to(elapsed);

            let current_position = match &tween.command {
                TurtleCommand::Circle {
                    radius,
                    angle,
                    direction,
                    ..
                } => {
                    let angle_traveled = angle.to_radians() * progress;
                    calculate_circle_position(
                        tween.start_params.position,
                        tween.start_params.heading,
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

            state.params.position = current_position;
            tween.current_position = current_position;

            // Heading changes proportionally with progress for all commands
            let current_heading = normalize_angle(match &tween.command {
                TurtleCommand::Circle {
                    angle, direction, ..
                } => match direction {
                    CircleDirection::Left => {
                        tween.start_params.heading - angle.to_radians() * progress
                    }
                    CircleDirection::Right => {
                        tween.start_params.heading + angle.to_radians() * progress
                    }
                },
                TurtleCommand::Turn(angle) => {
                    tween.start_params.heading + angle.to_radians() * progress
                }
                _ => {
                    // For other commands that change heading, lerp directly
                    let heading_diff = tween.target_params.heading - tween.start_params.heading;
                    tween.start_params.heading + heading_diff * progress
                }
            });

            state.params.heading = current_heading;
            tween.current_heading = current_heading;
            state.params.pen_width = tween.pen_width_tweener.move_to(elapsed);

            // Discrete properties (switch at 50% progress)
            let progress = (elapsed / tween.duration).min(1.0);
            if progress >= 0.5 {
                state.params.pen_down = tween.target_params.pen_down;
                state.params.color = tween.target_params.color;
                state.params.fill_color = tween.target_params.fill_color;
                state.params.visible = tween.target_params.visible;
                state.params.shape = tween.target_params.shape.clone();
            }

            // Check if tween is finished (use heading_tweener as it's used by all commands)
            if tween.heading_tweener.is_finished() {
                let start_params = tween.start_params.clone();
                let target_params = tween.target_params.clone();
                let command = tween.command.clone();

                // Drop the mutable borrow of tween before mutably borrowing state
                state.params = target_params.clone();

                crate::execution::record_fill_vertices_after_movement(
                    &command,
                    &start_params,
                    state,
                );

                state.tween_controller.current_tween = None;

                // Execute side-effect-only commands using centralized helper
                if crate::execution::execute_command_side_effects(&command, state) {
                    return Self::update(state); // Continue to next command
                }

                // Return drawable commands using the original start and target params
                if Self::command_creates_drawing(&command) && start_params.pen_down {
                    return vec![(command, start_params.clone(), target_params.clone())];
                }

                return Self::update(state); // Continue to next command
            }

            return Vec::new();
        }

        // Start next tween
        if let Some(command) = state.tween_controller.queue.next() {
            let command_clone = command.clone();

            // Handle commands that should execute immediately (no animation)
            match &command_clone {
                TurtleCommand::SetSpeed(new_speed) => {
                    state.set_speed(*new_speed);
                    state.tween_controller.speed = *new_speed;
                    if matches!(state.tween_controller.speed, AnimationSpeed::Instant(_)) {
                        return Self::update(state);
                    }
                    return Self::update(state);
                }
                _ => {
                    // Use centralized helper for side effects
                    if crate::execution::execute_command_side_effects(&command_clone, state) {
                        return Self::update(state);
                    }
                }
            }

            let speed = state.tween_controller.speed; // Extract speed before borrowing self
            let duration = Self::calculate_duration_with_state(&command_clone, state, speed);

            // Calculate target state
            let target_state = Self::calculate_target_state(&state.params, &command_clone);

            // Create tweeners for smooth animation
            let position_tweener = Tweener::new(
                TweenVec2::from(state.params.position),
                TweenVec2::from(target_state.position),
                duration,
                CubicInOut,
            );

            let heading_tweener = Tweener::new(
                0.0, // We'll handle angle wrapping separately
                1.0, duration, CubicInOut,
            );

            let pen_width_tweener = Tweener::new(
                state.params.pen_width,
                target_state.pen_width,
                duration,
                CubicInOut,
            );

            state.tween_controller.current_tween = Some(CommandTween {
                turtle_id: state.turtle_id,
                command: command_clone,
                start_time: get_time(),
                duration,
                start_params: state.params.clone(),
                target_params: target_state.clone(),
                current_position: state.params.position,
                current_heading: state.params.heading,
                position_tweener,
                heading_tweener,
                pen_width_tweener,
            });
        }

        Vec::new()
    }

    #[must_use]
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

    fn calculate_duration_with_state(
        command: &TurtleCommand,
        current: &Turtle,
        speed: AnimationSpeed,
    ) -> f64 {
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
            TurtleCommand::Goto(target) => {
                // Calculate actual distance from current position to target
                let dx = target.x - current.params.position.x;
                let dy = target.y - current.params.position.y;
                let distance = (dx * dx + dy * dy).sqrt();
                distance / speed
            }
            _ => 0.0, // Instant commands
        };
        f64::from(base_time.max(0.01)) // Minimum duration
    }

    fn calculate_target_state(current: &TurtleParams, command: &TurtleCommand) -> TurtleParams {
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
                // Flip Y coordinate: turtle graphics uses Y+ = up, but Macroquad uses Y+ = down
                target.position = vec2(coord.x, -coord.y);
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
            TurtleCommand::BeginFill | TurtleCommand::EndFill | TurtleCommand::WriteText { .. } => {
                // Fill and text commands don't change turtle state for tweening purposes
                // They're handled directly in execution
            }
            TurtleCommand::Reset => {
                // Reset returns to default state
                target = TurtleParams::default();
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
