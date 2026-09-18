//! Tweening system for smooth animations

use crate::circle_geometry::{CircleDirection, CircleGeometry};
use crate::commands::{CommandQueue, TurtleCommand};
use crate::general::{AnimationSpeed, Radians};
use crate::state::{DrawCommand, FillState, TurtleParams};
use macroquad::prelude::*;
use std::collections::VecDeque;
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
pub(crate) struct TweenController {
    queue: VecDeque<TurtleCommand>,
    current_tween: Option<CommandTween>,
    speed: AnimationSpeed,
}

#[derive(Clone, Debug)]
pub(crate) struct CommandTween {
    pub(crate) turtle_id: usize,
    pub(crate) command: TurtleCommand,
    pub(crate) start_time: f64,
    pub(crate) duration: f64,
    pub(crate) start_params: TurtleParams,
    pub(crate) target_params: TurtleParams,
    pub(crate) current_position: Vec2,
    pub(crate) current_heading: f32,
    position_tweener: Tweener<TweenVec2, f64, CubicInOut>,
    heading_tweener: Tweener<f32, f64, CubicInOut>,
    pen_width_tweener: Tweener<f32, f64, CubicInOut>,
}

impl TweenController {
    #[must_use]
    pub fn new(queue: CommandQueue, speed: AnimationSpeed) -> Self {
        Self {
            queue: queue.into_iter().collect(),
            current_tween: None,
            speed,
        }
    }

    pub fn set_speed(&mut self, speed: AnimationSpeed) {
        self.speed = speed;
    }

    /// Append commands to the queue.
    ///
    /// Consumed commands are removed as they execute, and new commands
    /// are queued at the back.
    pub fn append_commands(&mut self, new_queue: CommandQueue) {
        self.queue.extend(new_queue);
    }

    /// Drive the animation controller for one frame.
    ///
    /// Returns `(command, start_params, end_params)` for every command that
    /// completed this frame and whose stroke needs to be tessellated by the
    /// caller.
    ///
    /// By accepting `params`, `filling`, and `commands` as separate mutable
    /// borrows the caller can split `&mut Turtle` into disjoint field borrows,
    /// eliminating the old static-method borrow-checker workaround.
    #[allow(clippy::too_many_lines)]
    pub fn update(
        &mut self,
        turtle_id: usize,
        params: &mut TurtleParams,
        filling: &mut Option<FillState>,
        commands: &mut Vec<DrawCommand>,
        svg_log: &mut crate::state::SvgLog,
    ) -> Vec<(TurtleCommand, TurtleParams, TurtleParams)> {
        // In instant mode, execute commands up to the draw calls per frame limit
        if let AnimationSpeed::Instant(max_draw_calls) = self.speed {
            let mut completed_commands: Vec<(TurtleCommand, TurtleParams, TurtleParams)> =
                Vec::new();
            let mut draw_call_count = 0;

            // Consume commands from the front of the queue
            while let Some(command) = self.queue.pop_front() {
                // Handle SetSpeed command to potentially switch modes
                if let TurtleCommand::SetSpeed(new_speed) = &command {
                    params.speed = *new_speed;
                    self.speed = *new_speed;
                    if matches!(self.speed, AnimationSpeed::Animated(_)) {
                        break;
                    }
                    continue;
                }

                // Execute side-effect-only commands using centralized helper
                if crate::execution::execute_command_side_effects(
                    &command, turtle_id, params, filling, commands, svg_log,
                ) {
                    continue; // Command fully handled
                }

                // Save start state and compute target state
                let start_params = params.clone();
                let target_params = Self::calculate_target_state(&start_params, &command);

                // Update state to the target (instant execution)
                *params = target_params.clone();

                // Record fill vertices AFTER movement
                crate::execution::record_fill_vertices_after_movement(
                    &command,
                    &start_params,
                    turtle_id,
                    params,
                    filling,
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
        if let Some(ref mut tween) = self.current_tween {
            let elapsed = current_time() - tween.start_time;

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
                    let angle_traveled = angle.as_radians().value() * progress;
                    calculate_circle_position(
                        tween.start_params.position,
                        Radians::new(tween.start_params.heading),
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

            params.position = current_position;
            tween.current_position = current_position;

            // Heading changes proportionally with progress for all commands
            let current_heading = normalize_angle(match &tween.command {
                TurtleCommand::Circle {
                    angle, direction, ..
                } => match direction {
                    CircleDirection::Left => {
                        tween.start_params.heading - angle.as_radians().value() * progress
                    }
                    CircleDirection::Right => {
                        tween.start_params.heading + angle.as_radians().value() * progress
                    }
                },
                TurtleCommand::Turn(angle) => {
                    tween.start_params.heading + angle.as_radians().value() * progress
                }
                _ => {
                    // For other commands that change heading, lerp directly
                    let heading_diff = tween.target_params.heading - tween.start_params.heading;
                    tween.start_params.heading + heading_diff * progress
                }
            });

            params.heading = current_heading;
            tween.current_heading = current_heading;
            params.pen_width = tween.pen_width_tweener.move_to(elapsed);

            // Discrete properties (switch at 50% progress)
            let progress = (elapsed / tween.duration).min(1.0);
            if progress >= 0.5 {
                params.pen_down = tween.target_params.pen_down;
                params.color = tween.target_params.color;
                params.fill_color = tween.target_params.fill_color;
                params.visible = tween.target_params.visible;
                params.shape = tween.target_params.shape.clone();
            }

            // Check if tween is finished (use heading_tweener as it's used by all commands)
            if tween.heading_tweener.is_finished() {
                let start_params = tween.start_params.clone();
                let target_params = tween.target_params.clone();
                let command = tween.command.clone();

                // tween borrow ends here (NLL) — safe to reassign self.current_tween below
                *params = target_params.clone();

                crate::execution::record_fill_vertices_after_movement(
                    &command,
                    &start_params,
                    turtle_id,
                    params,
                    filling,
                );

                self.current_tween = None;

                // Execute side-effect-only commands using centralized helper
                if crate::execution::execute_command_side_effects(
                    &command, turtle_id, params, filling, commands, svg_log,
                ) {
                    return self.update(turtle_id, params, filling, commands, svg_log);
                }

                // Return drawable commands using the original start and target params
                if Self::command_creates_drawing(&command) && start_params.pen_down {
                    return vec![(command, start_params.clone(), target_params.clone())];
                }

                return self.update(turtle_id, params, filling, commands, svg_log);
            }

            return Vec::new();
        }

        // Start next tween
        if let Some(command) = self.queue.pop_front() {
            // Handle commands that should execute immediately (no animation)
            match &command {
                TurtleCommand::SetSpeed(new_speed) => {
                    params.speed = *new_speed;
                    self.speed = *new_speed;
                    if matches!(self.speed, AnimationSpeed::Instant(_)) {
                        return self.update(turtle_id, params, filling, commands, svg_log);
                    }
                    return self.update(turtle_id, params, filling, commands, svg_log);
                }
                _ => {
                    // Use centralized helper for side effects
                    if crate::execution::execute_command_side_effects(
                        &command, turtle_id, params, filling, commands, svg_log,
                    ) {
                        return self.update(turtle_id, params, filling, commands, svg_log);
                    }
                }
            }

            let speed = self.speed;
            let duration = Self::calculate_duration_with_state(&command, params, speed);

            // Calculate target state
            let target_state = Self::calculate_target_state(params, &command);

            // Create tweeners for smooth animation
            let position_tweener = Tweener::new(
                TweenVec2::from(params.position),
                TweenVec2::from(target_state.position),
                duration,
                CubicInOut,
            );

            let heading_tweener = Tweener::new(
                0.0, // We'll handle angle wrapping separately
                1.0, duration, CubicInOut,
            );

            let pen_width_tweener = Tweener::new(
                params.pen_width,
                target_state.pen_width,
                duration,
                CubicInOut,
            );

            self.current_tween = Some(CommandTween {
                turtle_id,
                command,
                start_time: current_time(),
                duration,
                start_params: params.clone(),
                target_params: target_state.clone(),
                current_position: params.position,
                current_heading: params.heading,
                position_tweener,
                heading_tweener,
                pen_width_tweener,
            });
        }

        Vec::new()
    }

    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.current_tween.is_none() && self.queue.is_empty()
    }

    /// Get the current active tween if one is in progress
    pub(crate) fn current_tween(&self) -> Option<&CommandTween> {
        self.current_tween.as_ref()
    }

    fn command_creates_drawing(command: &TurtleCommand) -> bool {
        command.produces_drawing()
    }

    fn calculate_duration_with_state(
        command: &TurtleCommand,
        params: &TurtleParams,
        speed: AnimationSpeed,
    ) -> f64 {
        command.animation_duration(params, speed)
    }

    fn calculate_target_state(current: &TurtleParams, command: &TurtleCommand) -> TurtleParams {
        let mut target = current.clone();
        command.apply_to_params(&mut target);
        target
    }
}

/// Calculate position on a circular arc.
///
/// `start_heading` is in radians (typed as `Radians` to make the unit
/// explicit at every call site). `angle_traveled` is already a raw `f32`
/// radians value produced by multiplying `Degrees::as_radians().value()`
/// by a tween progress scalar.
fn calculate_circle_position(
    start_pos: Vec2,
    start_heading: Radians,
    radius: f32,
    angle_traveled: f32,
    direction: CircleDirection,
) -> Vec2 {
    let geom = CircleGeometry::new(start_pos, start_heading, radius, direction);
    geom.position_at_angle(angle_traveled)
}

/// Normalize angle to range [-PI, PI] to prevent floating-point drift
pub(crate) fn normalize_angle(angle: f32) -> f32 {
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

#[inline]
fn current_time() -> f64 {
    #[cfg(test)]
    {
        use std::sync::OnceLock;
        use std::time::Instant;
        static START: OnceLock<Instant> = OnceLock::new();
        START.get_or_init(Instant::now).elapsed().as_secs_f64()
    }
    #[cfg(not(test))]
    {
        macroquad::time::get_time()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::TurtleCommand;
    use crate::general::Degrees;
    use crate::state::TurtleParams;

    fn make_test_params() -> TurtleParams {
        TurtleParams {
            position: vec2(0.0, 0.0),
            heading: 0.0,
            pen_down: true,
            pen_width: 1.0,
            color: Color::new(0.0, 0.0, 0.0, 1.0),
            fill_color: None,
            visible: true,
            shape: crate::shapes::TurtleShape::turtle(),
            speed: AnimationSpeed::Instant(100),
        }
    }

    #[test]
    fn test_instant_mode_drains_queue() {
        let mut queue = CommandQueue::new();
        queue.push(TurtleCommand::Move(100.0));
        queue.push(TurtleCommand::Turn(Degrees::new(90.0)));
        queue.push(TurtleCommand::PenUp);
        queue.push(TurtleCommand::Move(50.0));

        let mut controller = TweenController::new(queue, AnimationSpeed::Instant(100));
        assert_eq!(controller.queue.len(), 4);
        assert!(!controller.is_complete());

        let mut params = make_test_params();
        let mut filling = None;
        let mut commands = Vec::new();
        let mut svg_log = crate::state::SvgLog::default();

        let completed = controller.update(0, &mut params, &mut filling, &mut commands, &mut svg_log);

        assert_eq!(controller.queue.len(), 0, "Queue must be empty after instant update");
        assert!(controller.is_complete(), "Controller must be complete when queue is drained");
        assert!(!completed.is_empty());
    }

    #[test]
    fn test_streaming_append_commands_does_not_accumulate() {
        let mut controller = TweenController::new(CommandQueue::new(), AnimationSpeed::Instant(100));
        let mut params = make_test_params();
        let mut filling = None;
        let mut commands = Vec::new();
        let mut svg_log = crate::state::SvgLog::default();

        // Simulate streaming commands across 50 frames (like clock_threaded)
        for _ in 0..50 {
            let mut batch = CommandQueue::new();
            batch.push(TurtleCommand::Reset);
            batch.push(TurtleCommand::PenDown);
            batch.push(TurtleCommand::Move(10.0));
            batch.push(TurtleCommand::Turn(Degrees::new(30.0)));

            controller.append_commands(batch);
            assert_eq!(controller.queue.len(), 4);

            controller.update(0, &mut params, &mut filling, &mut commands, &mut svg_log);

            // Verify queue is pruned back to 0 — no memory leak / accumulation
            assert_eq!(controller.queue.len(), 0);
            assert!(controller.is_complete());
        }
    }

    #[test]
    fn test_instant_mode_respects_batch_limit_and_retains_pending() {
        let mut queue = CommandQueue::new();
        // 5 drawing commands
        queue.push(TurtleCommand::Move(10.0));
        queue.push(TurtleCommand::Move(20.0));
        queue.push(TurtleCommand::Move(30.0));
        queue.push(TurtleCommand::Move(40.0));
        queue.push(TurtleCommand::Move(50.0));

        // Limit to 2 draw calls per frame
        let mut controller = TweenController::new(queue, AnimationSpeed::Instant(2));
        let mut params = make_test_params();
        let mut filling = None;
        let mut commands = Vec::new();
        let mut svg_log = crate::state::SvgLog::default();

        // Frame 1: processes 2 drawing commands
        let completed1 = controller.update(0, &mut params, &mut filling, &mut commands, &mut svg_log);
        assert_eq!(completed1.len(), 2);
        assert_eq!(controller.queue.len(), 3, "3 commands should remain in queue");
        assert!(!controller.is_complete());

        // Frame 2: processes next 2 drawing commands
        let completed2 = controller.update(0, &mut params, &mut filling, &mut commands, &mut svg_log);
        assert_eq!(completed2.len(), 2);
        assert_eq!(controller.queue.len(), 1, "1 command should remain in queue");
        assert!(!controller.is_complete());

        // Frame 3: processes last drawing command
        let completed3 = controller.update(0, &mut params, &mut filling, &mut commands, &mut svg_log);
        assert_eq!(completed3.len(), 1);
        assert_eq!(controller.queue.len(), 0, "Queue must be completely drained");
        assert!(controller.is_complete());
    }

    #[test]
    fn test_animated_mode_pops_to_current_tween() {
        let mut queue = CommandQueue::new();
        queue.push(TurtleCommand::Move(100.0));
        queue.push(TurtleCommand::Move(50.0));

        let mut controller = TweenController::new(
            queue,
            AnimationSpeed::Animated(100.0),
        );
        assert_eq!(controller.queue.len(), 2);
        assert!(!controller.is_complete());

        let mut params = make_test_params();
        let mut filling = None;
        let mut commands = Vec::new();
        let mut svg_log = crate::state::SvgLog::default();

        // Calling update should pop the first command into current_tween
        controller.update(0, &mut params, &mut filling, &mut commands, &mut svg_log);

        assert_eq!(controller.queue.len(), 1, "First command must be popped into current_tween");
        assert!(controller.current_tween().is_some());
        assert!(!controller.is_complete());
    }
}
