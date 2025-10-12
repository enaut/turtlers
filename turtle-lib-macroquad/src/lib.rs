//! Turtle graphics library for Macroquad
//!
//! This library provides a turtle graphics API for creating drawings and animations
//! using the Macroquad game framework.
//!
//! # Example
//! ```no_run
//! use macroquad::prelude::*;
//! use turtle_lib_macroquad::*;
//!
//! #[macroquad::main("Turtle")]
//! async fn main() {
//!     let mut plan = create_turtle();
//!     plan.forward(100.0).right(90.0).forward(100.0);
//!     
//!     let mut app = TurtleApp::new().with_commands(plan.build());
//!     
//!     loop {
//!         clear_background(WHITE);
//!         app.update();
//!         app.render();
//!         next_frame().await
//!     }
//! }
//! ```

pub mod builders;
pub mod circle_geometry;
pub mod commands;
pub mod drawing;
pub mod execution;
pub mod general;
pub mod shapes;
pub mod state;
pub mod tessellation;
pub mod tweening;

// Re-export commonly used types
pub use builders::{CurvedMovement, DirectionalMovement, Turnable, TurtlePlan, WithCommands};
pub use commands::{CommandQueue, TurtleCommand};
pub use general::{Angle, AnimationSpeed, Color, Coordinate, Length, Precision};
pub use shapes::{ShapeType, TurtleShape};
pub use state::{DrawCommand, TurtleState, TurtleWorld};
pub use tweening::TweenController;

use macroquad::prelude::*;

/// Main turtle application struct
pub struct TurtleApp {
    world: TurtleWorld,
    tween_controller: Option<TweenController>,
    speed: AnimationSpeed,
    // Mouse panning state
    is_dragging: bool,
    last_mouse_pos: Option<Vec2>,
    // Zoom state
    zoom_level: f32,
}

impl TurtleApp {
    /// Create a new TurtleApp with default settings
    pub fn new() -> Self {
        Self {
            world: TurtleWorld::new(),
            tween_controller: None,
            speed: AnimationSpeed::default(),
            is_dragging: false,
            last_mouse_pos: None,
            zoom_level: 1.0,
        }
    }

    /// Add commands to the turtle
    ///
    /// Speed is controlled by SetSpeed commands in the queue.
    /// Use `set_speed()` on the turtle plan to set animation speed.
    /// Speed >= 999 = instant mode, speed < 999 = animated mode.
    ///
    /// # Arguments
    /// * `queue` - The command queue to execute
    pub fn with_commands(mut self, queue: CommandQueue) -> Self {
        // The TweenController will switch between instant and animated mode
        // based on SetSpeed commands encountered
        self.tween_controller = Some(TweenController::new(queue, self.speed));
        self
    }

    /// Update animation state (call every frame)
    pub fn update(&mut self) {
        // Handle mouse panning and zoom
        self.handle_mouse_panning();
        self.handle_mouse_zoom();

        if let Some(ref mut controller) = self.tween_controller {
            let completed_commands =
                controller.update(&mut self.world.turtle, &mut self.world.commands);

            // Process all completed commands (multiple in instant mode, 0-1 in animated mode)
            for (completed_cmd, start_state, end_state) in completed_commands {
                // Add draw commands for the completed tween
                execution::add_draw_for_completed_tween(
                    &completed_cmd,
                    &start_state,
                    &end_state,
                    &mut self.world,
                );
            }
        }
    }
    /// Handle mouse click and drag for panning
    fn handle_mouse_panning(&mut self) {
        let mouse_pos = mouse_position();
        let mouse_pos = vec2(mouse_pos.0, mouse_pos.1);

        if is_mouse_button_pressed(MouseButton::Left) {
            self.is_dragging = true;
            self.last_mouse_pos = Some(mouse_pos);
        }

        if is_mouse_button_released(MouseButton::Left) {
            self.is_dragging = false;
            self.last_mouse_pos = None;
        }

        if self.is_dragging {
            if let Some(last_pos) = self.last_mouse_pos {
                // Calculate delta in screen space
                let delta = mouse_pos - last_pos;

                // Convert screen delta to world space delta
                // The camera zoom is 2.0 / screen_width, so world_units = screen_pixels / (screen_size * zoom / 2)
                let world_delta = vec2(
                    -delta.x, -delta.y, // Flip Y because screen Y is down
                );

                self.world.camera.target += world_delta * self.zoom_level;
            }
            self.last_mouse_pos = Some(mouse_pos);
        }
    }

    /// Handle mouse wheel for zooming
    fn handle_mouse_zoom(&mut self) {
        let (_wheel_x, wheel_y) = mouse_wheel();

        if wheel_y != 0.0 {
            // Zoom factor: positive wheel_y = zoom in, negative = zoom out
            let zoom_factor = 1.0 + wheel_y * 0.1;
            self.zoom_level *= zoom_factor;

            // Clamp zoom level to reasonable values
            self.zoom_level = self.zoom_level.clamp(0.1, 10.0);
        }
    }

    /// Render the turtle world (call every frame)
    pub fn render(&self) {
        // Get active tween if in animated mode
        let active_tween = self
            .tween_controller
            .as_ref()
            .and_then(|c| c.current_tween());
        drawing::render_world_with_tween(&self.world, active_tween, self.zoom_level);
    }

    /// Check if all commands have been executed
    pub fn is_complete(&self) -> bool {
        self.tween_controller
            .as_ref()
            .map(|c| c.is_complete())
            .unwrap_or(true)
    }

    /// Get reference to the world state
    pub fn world(&self) -> &TurtleWorld {
        &self.world
    }

    /// Get mutable reference to the world state
    pub fn world_mut(&mut self) -> &mut TurtleWorld {
        &mut self.world
    }
}

impl Default for TurtleApp {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to create a new turtle plan
///
/// # Example
/// ```
/// use turtle_lib_macroquad::*;
///
/// let mut turtle = create_turtle();
/// turtle.forward(100.0).right(90.0).forward(50.0);
/// let commands = turtle.build();
/// ```
pub fn create_turtle() -> TurtlePlan {
    TurtlePlan::new()
}

/// Convenience function to get a turtle plan (alias for create_turtle)
pub fn get_a_turtle() -> TurtlePlan {
    create_turtle()
}
