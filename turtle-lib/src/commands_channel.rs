//! Per-turtle command channels for multi-threaded game logic
//!
//! Enables sending turtle commands from game logic threads to the render thread
//! without blocking the render loop.
//!
//! # Usage
//!
//! ```no_run
//! use turtle_lib::*;
//! use std::thread;
//! use macroquad::prelude::{next_frame, clear_background, WHITE};
//! # #[macroquad::main("Threading")]
//! # async fn main() {
//! let mut app = TurtleApp::new();
//!
//! // Create a turtle and get its command sender
//! let turtle_tx = app.create_turtle_channel(100);
//!
//! // Spawn a game logic thread
//! thread::spawn({
//!     let tx = turtle_tx.clone();
//!     move || {
//!         let mut plan = create_turtle_plan();
//!         plan.forward(100.0).right(90.0);
//!         tx.send(plan.build()).ok();
//!     }
//! });
//!
//! // Main render loop
//! loop {
//!     clear_background(WHITE);
//!     app.process_commands();
//!     app.update();
//!     app.render();
//!     next_frame().await;
//! }
//! # }
//! ```

use crate::commands::CommandQueue;
use crossbeam::channel::{bounded, Receiver, Sender};

/// Sender for turtle commands from a game logic thread
///
/// This is tied to a specific turtle created via `TurtleApp::create_turtle_channel()`.
/// The turtle is guaranteed to exist on the render thread.
///
/// # Thread Safety
/// Can be cloned and shared across threads. Multiple game threads can send
/// commands to the same turtle safely.
///
/// # Examples
/// ```no_run
/// # use turtle_lib::*;
/// # fn example() -> Result<(), String> {
/// # let mut app = TurtleApp::new();
/// let tx = app.create_turtle_channel(100);
///
/// // Send commands from game thread
/// let mut plan = create_turtle_plan();
/// plan.forward(50.0);
/// tx.send(plan.clone().build())?;
///
/// // Or non-blocking variant
/// tx.try_send(plan.build()).ok();
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct TurtleCommandSender {
    turtle_id: usize,
    tx: Sender<CommandQueue>,
}

/// Receiver for turtle commands on the render thread
///
/// Paired with `TurtleCommandSender` via `turtle_command_channel()`.
/// Automatically managed by `TurtleApp::process_commands()`.
pub struct TurtleCommandReceiver {
    turtle_id: usize,
    rx: Receiver<CommandQueue>,
}

impl TurtleCommandSender {
    /// Get the turtle ID this sender is bound to
    #[must_use]
    pub fn turtle_id(&self) -> usize {
        self.turtle_id
    }

    /// Send commands (blocking)
    ///
    /// Blocks if the channel buffer is full. This is appropriate for game logic
    /// threads where blocking is acceptable. The buffer size is specified when
    /// creating the channel.
    ///
    /// # Errors
    /// Returns error if the receiver has been dropped (render thread exited).
    ///
    /// # Examples
    /// ```no_run
    /// # use turtle_lib::*;
    /// # fn example() -> Result<(), String> {
    /// # let mut app = TurtleApp::new();
    /// # let tx = app.create_turtle_channel(100);
    /// let mut plan = create_turtle_plan();
    /// plan.forward(100.0);
    /// tx.send(plan.build())?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn send(&self, queue: CommandQueue) -> Result<(), String> {
        self.tx
            .send(queue)
            .map_err(|e| format!("Channel disconnected: {}", e))
    }

    /// Send commands (non-blocking)
    ///
    /// Returns immediately. If the channel buffer is full, returns an error
    /// without blocking.
    ///
    /// # Errors
    /// Returns error if the buffer is full or the receiver has been dropped.
    ///
    /// # Examples
    /// ```no_run
    /// # use turtle_lib::*;
    /// # fn example() {
    /// # let mut app = TurtleApp::new();
    /// # let tx = app.create_turtle_channel(100);
    /// let mut plan = create_turtle_plan();
    /// plan.forward(100.0);
    /// tx.try_send(plan.build()).ok();  // Ignore if buffer full
    /// # }
    /// ```
    pub fn try_send(&self, queue: CommandQueue) -> Result<(), String> {
        self.tx
            .try_send(queue)
            .map_err(|e| format!("Failed to send: {}", e))
    }
}

impl TurtleCommandReceiver {
    /// Get the turtle ID this receiver is bound to
    #[must_use]
    pub fn turtle_id(&self) -> usize {
        self.turtle_id
    }

    /// Drain all pending commands for this turtle (non-blocking)
    ///
    /// # Examples
    /// ```no_run
    /// # use turtle_lib::*;
    /// # async fn example() {
    /// # let mut app = TurtleApp::new();
    /// # let _tx = app.create_turtle_channel(100);
    /// // This is called automatically by app.process_commands()
    /// // But you can also do it manually:
    /// loop {
    ///     app.update();
    ///     app.render();
    ///     # break;
    /// }
    /// # }
    /// ```
    pub fn recv_all(&self) -> Vec<CommandQueue> {
        self.rx.try_iter().collect()
    }

    /// Try to receive one command batch (non-blocking)
    #[must_use]
    pub fn try_recv(&self) -> Option<CommandQueue> {
        self.rx.try_recv().ok()
    }

    /// Check if this receiver's queue is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rx.is_empty()
    }

    /// Get the number of pending command batches
    #[must_use]
    pub fn len(&self) -> usize {
        self.rx.len()
    }
}

/// Create a command channel for a specific turtle
///
/// The tuple represents (sender, receiver) where:
/// - Sender goes to game logic threads (cloneable, can be distributed)
/// - Receiver stays in the render thread (part of TurtleApp internally)
///
/// # Arguments
/// * `turtle_id` - The ID of the turtle this channel is for (must be valid)
/// * `buffer_size` - Maximum number of pending command batches before sender blocks
///
/// # Panics
/// Panics if buffer_size is 0.
///
/// # Examples
/// ```no_run
/// # use turtle_lib::*;
/// # fn example() {
/// let (tx, _rx) = turtle_command_channel(0, 100);
/// // Sender goes to game threads
/// // Receiver stays in render thread (or TurtleApp)
/// # }
/// ```
pub fn turtle_command_channel(
    turtle_id: usize,
    buffer_size: usize,
) -> (TurtleCommandSender, TurtleCommandReceiver) {
    assert!(buffer_size > 0, "buffer_size must be > 0");
    let (tx, rx) = bounded(buffer_size);
    (
        TurtleCommandSender { turtle_id, tx },
        TurtleCommandReceiver { turtle_id, rx },
    )
}
