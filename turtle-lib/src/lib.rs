//! Turtle graphics library for Macroquad
//!
//! This library provides a turtle graphics API for creating drawings and animations
//! using the Macroquad game framework.
//!
//! # Quick Start with `turtle_main` Macro
//!
//! The easiest way to create a turtle program is using the `turtle_main` macro:
//!
//! ```no_run
//! use macroquad::prelude::*;
//! use turtle_lib::*;
//!
//! #[turtle_main("My Drawing")]
//! fn draw(turtle: &mut TurtlePlan) {
//!     turtle.set_pen_color(RED);
//!     turtle.forward(100.0);
//!     turtle.right(90.0);
//!     turtle.forward(100.0);
//! }
//! ```
//!
//! The macro automatically handles window setup, rendering loop, and quit handling.
//!
//! # Manual Setup Example
//!
//! For more control, you can set up the application manually:
//!
//! ```no_run
//! use macroquad::prelude::*;
//! use turtle_lib::*;
//!
//! #[macroquad::main("Turtle")]
//! async fn main() {
//!     let mut plan = create_turtle_plan();
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
pub mod commands_channel;
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
pub use commands_channel::{turtle_command_channel, TurtleCommandReceiver, TurtleCommandSender};
pub use general::{Angle, AnimationSpeed, Color, Coordinate, Length, Precision};
pub use shapes::{ShapeType, TurtleShape};
pub use state::{DrawCommand, Turtle, TurtleWorld};
pub use tweening::TweenController;

pub mod export;
#[cfg(feature = "svg")]
pub mod export_svg;

// Re-export the turtle_main macro
pub use turtle_lib_macros::turtle_main;

// Re-export common macroquad types and colors for convenience
pub use macroquad::prelude::{
    vec2, BLACK, BLUE, DARKGRAY, GOLD, GREEN, ORANGE, PURPLE, RED, WHITE, YELLOW,
};

use macroquad::prelude::*;
use std::collections::HashMap;

/// Main turtle application struct
pub struct TurtleApp {
    world: TurtleWorld,
    // Receivers for turtle command channels
    receivers: HashMap<usize, TurtleCommandReceiver>,
    // Mouse panning state
    is_dragging: bool,
    last_mouse_pos: Option<Vec2>,
    // Zoom state
    zoom_level: f32,
}

impl TurtleApp {
    /// Exportiere das aktuelle Drawing in das gewünschte Format
    #[allow(unused_variables)]
    /// Export the current drawing to a file in the specified format
    ///
    /// # Errors
    ///
    /// Returns an error if the export fails (e.g., unsupported format, file I/O error)
    pub fn export_drawing(
        &self,
        filename: &str,
        format: export::DrawingFormat,
    ) -> Result<(), export::ExportError> {
        match format {
            #[cfg(feature = "svg")]
            export::DrawingFormat::Svg => {
                use crate::export::DrawingExporter;
                use export_svg::svg_export::SvgExporter;
                let exporter = SvgExporter;
                exporter.export(&self.world, filename)
            }
            // Weitere Formate können hier ergänzt werden
            #[allow(unreachable_patterns)]
            _ => Err(export::ExportError::Format(
                "Export-Format nicht unterstützt".to_string(),
            )),
        }
    }
    /// Create a new `TurtleApp` with default settings
    #[must_use]
    pub fn new() -> Self {
        Self {
            world: TurtleWorld::new(),
            receivers: HashMap::new(),
            is_dragging: false,
            last_mouse_pos: None,
            zoom_level: 1.0,
        }
    }

    /// Add a new turtle and return its ID
    pub fn add_turtle(&mut self) -> usize {
        self.world.add_turtle()
    }

    /// Create a turtle and a command channel for it
    ///
    /// This is the preferred way to set up turtles when using threading.
    /// Call this ONCE per turtle during setup, before spawning game logic threads.
    ///
    /// # Arguments
    /// * `buffer_size` - Maximum pending command batches before sender blocks (typically 50-200)
    ///
    /// # Returns
    /// A `TurtleCommandSender` that can be cloned and sent to game logic threads.
    /// The turtle is automatically managed by `TurtleApp`.
    ///
    /// # Examples
    /// ```no_run
    /// # use turtle_lib::*;
    /// # #[macroquad::main("Threading")]
    /// # async fn main() {
    /// let mut app = TurtleApp::new();
    ///
    /// // Create turtle and get sender
    /// let turtle_tx = app.create_turtle_channel(100);
    ///
    /// // Send to game threads
    /// let tx_clone = turtle_tx.clone();
    /// std::thread::spawn(move || {
    ///     let mut plan = create_turtle_plan();
    ///     plan.forward(100.0);
    ///     tx_clone.send(plan.build()).ok();
    /// });
    /// # }
    /// ```
    pub fn create_turtle_channel(&mut self, buffer_size: usize) -> TurtleCommandSender {
        let turtle_id = self.world.add_turtle();
        let (tx, rx) = commands_channel::turtle_command_channel(turtle_id, buffer_size);
        self.receivers.insert(turtle_id, rx);
        tx
    }

    /// Process all pending commands from all turtle channels
    ///
    /// Call this once per frame in your render loop, before `update()`.
    /// Drains all receivers and applies commands to their respective turtles.
    ///
    /// # Examples
    /// ```no_run
    /// # use turtle_lib::*;
    /// # use macroquad::prelude::{next_frame, clear_background, WHITE};
    /// # #[macroquad::main("Threading")]
    /// # async fn main() {
    /// # let mut app = TurtleApp::new();
    /// # let _tx = app.create_turtle_channel(100);
    /// loop {
    ///     clear_background(WHITE);
    ///     app.process_commands();  // ← Process channel commands
    ///     app.update();
    ///     app.render();
    ///     next_frame().await;
    /// }
    /// # }
    /// ```
    pub fn process_commands(&mut self) {
        // Collect all turtle IDs to avoid borrow issues
        let turtle_ids: Vec<usize> = self.receivers.keys().copied().collect();

        for turtle_id in turtle_ids {
            if let Some(receiver) = self.receivers.get(&turtle_id) {
                for queue in receiver.recv_all() {
                    self.append_commands(turtle_id, queue);
                }
            }
        }
    }

    /// Add commands from a turtle plan to the application for the default turtle (ID 0)
    ///
    /// Speed is controlled by `SetSpeed` commands in the queue.
    /// Use `set_speed()` on the turtle plan to set animation speed.
    /// Speed >= 999 = instant mode, speed < 999 = animated mode.
    ///
    /// # Arguments
    /// * `queue` - The command queue to execute
    #[must_use]
    pub fn with_commands(self, queue: CommandQueue) -> Self {
        self.with_commands_for_turtle(0, queue)
    }

    /// Add commands from a turtle plan to the application for a specific turtle
    ///
    /// Speed is controlled by `SetSpeed` commands in the queue.
    /// Use `set_speed()` on the turtle plan to set animation speed.
    /// Speed >= 999 = instant mode, speed < 999 = animated mode.
    ///
    /// # Arguments
    /// * `turtle_id` - The ID of the turtle to control
    /// * `queue` - The command queue to execute
    #[must_use]
    pub fn with_commands_for_turtle(mut self, turtle_id: usize, queue: CommandQueue) -> Self {
        // Ensure turtle exists
        while self.world.turtles.len() <= turtle_id {
            self.world.add_turtle();
        }

        // Append commands to the turtle's controller
        if let Some(turtle) = self.world.get_turtle_mut(turtle_id) {
            turtle.tween_controller.append_commands(queue);
        }
        self
    }

    /// Execute a plan immediately on a specific turtle (no animation)
    pub fn execute_immediate(&mut self, turtle_id: usize, plan: TurtlePlan) {
        for ref cmd in plan.build() {
            execution::execute_command_with_id(cmd, turtle_id, &mut self.world);
        }
    }

    /// Append commands to a turtle's animation queue
    pub fn append_to_queue(&mut self, turtle_id: usize, plan: TurtlePlan) {
        // Ensure turtle exists
        while self.world.turtles.len() <= turtle_id {
            self.world.add_turtle();
        }

        if let Some(turtle) = self.world.get_turtle_mut(turtle_id) {
            turtle.tween_controller.append_commands(plan.build());
        }
    }

    /// Append commands from a `CommandQueue` to a turtle's animation queue
    ///
    /// Used internally by `process_commands()` and can be used directly
    /// when you have a `CommandQueue` instead of a `TurtlePlan`.
    pub fn append_commands(&mut self, turtle_id: usize, queue: CommandQueue) {
        // Ensure turtle exists
        while self.world.turtles.len() <= turtle_id {
            self.world.add_turtle();
        }

        if let Some(turtle) = self.world.get_turtle_mut(turtle_id) {
            turtle.tween_controller.append_commands(queue);
        }
    }

    /// Update animation state (call every frame)
    pub fn update(&mut self) {
        // Handle mouse panning and zoom
        self.handle_mouse_panning();
        self.handle_mouse_zoom();

        // Update all turtles' tween controllers
        for turtle in &mut self.world.turtles {
            // Extract draw_commands and controller temporarily to avoid borrow conflicts

            // Update the controller
            let completed_commands = TweenController::update(turtle);

            // Process all completed commands and add to the turtle's commands
            for (completed_cmd, tween_start, mut end_state) in completed_commands {
                let draw_command = execution::add_draw_for_completed_tween(
                    &completed_cmd,
                    &tween_start,
                    &mut end_state,
                );
                // Add the new draw commands to the turtle
                turtle.commands.extend(draw_command);
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
        drawing::render_world_with_tweens(&self.world, self.zoom_level);
    }

    /// Check if all commands have been executed
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.world
            .turtles
            .iter()
            .all(|turtle| turtle.tween_controller.is_complete())
    }

    /// Get reference to the world state
    #[must_use]
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
/// use turtle_lib::*;
///
/// let mut turtle = create_turtle_plan();
/// turtle.forward(100.0).right(90.0).forward(50.0);
/// let commands = turtle.build();
/// ```
#[must_use]
pub fn create_turtle_plan() -> TurtlePlan {
    TurtlePlan::new()
}
