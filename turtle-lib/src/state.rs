//! Turtle state and world state management

use crate::commands::CommandQueue;
use crate::general::{Angle, AnimationSpeed, Color, Coordinate};
use crate::shapes::TurtleShape;
use crate::tweening::TweenController;
use macroquad::prelude::*;

/// State during active fill operation
#[derive(Clone, Debug)]
pub struct FillState {
    /// Starting position of the fill
    pub start_position: Coordinate,

    /// All contours collected so far. Each contour is a separate closed path.
    /// The first contour is the outer boundary, subsequent contours are holes.
    pub contours: Vec<Vec<Coordinate>>,

    /// Current contour being built (vertices for the active `pen_down` segment)
    pub current_contour: Vec<Coordinate>,

    /// Fill color (cached from when `begin_fill` was called)
    pub fill_color: Color,
}

/// Parameters that define a turtle's visual state
#[derive(Clone, Debug)]
pub struct TurtleParams {
    pub position: Vec2,
    pub heading: f32,
    pub pen_down: bool,
    pub pen_width: f32,
    pub color: Color,
    pub fill_color: Option<Color>,
    pub visible: bool,
    pub shape: crate::shapes::TurtleShape,
    pub speed: AnimationSpeed,
}

impl Default for TurtleParams {
    /// Create `TurtleParams` from default values
    fn default() -> Self {
        Self {
            position: vec2(0.0, 0.0),
            heading: 0.0,
            pen_down: true,
            pen_width: 2.0,
            color: BLACK,
            fill_color: None,
            visible: true,
            shape: TurtleShape::turtle(),
            speed: AnimationSpeed::default(),
        }
    }
}

/// State of a single turtle
#[derive(Clone, Debug)]
pub struct Turtle {
    pub turtle_id: usize,
    pub params: TurtleParams,

    // Fill tracking
    pub filling: Option<FillState>,

    // Drawing commands created by this turtle
    pub commands: Vec<DrawCommand>,

    // Animation controller for this turtle
    pub tween_controller: TweenController,
}

impl Default for Turtle {
    fn default() -> Self {
        Self {
            turtle_id: 0,
            params: TurtleParams::default(),
            filling: None,
            commands: Vec::new(),
            tween_controller: TweenController::new(CommandQueue::new(), AnimationSpeed::default()),
        }
    }
}

impl Turtle {
    pub fn set_speed(&mut self, speed: AnimationSpeed) {
        self.params.speed = speed;
    }

    #[must_use]
    pub fn heading_angle(&self) -> Angle {
        Angle::radians(self.params.heading)
    }

    /// Reset turtle to default state (preserves `turtle_id` and queued commands)
    pub fn reset(&mut self) {
        // Clear all drawings
        self.commands.clear();

        // Clear fill state
        self.filling = None;

        // Reset parameters to defaults
        self.params = TurtleParams::default();

        // Keep turtle_id and tween_controller (preserves queued commands)
    }

    /// Start recording fill vertices
    pub fn begin_fill(&mut self, fill_color: Color) {
        self.filling = Some(FillState {
            start_position: self.params.position,
            contours: Vec::new(),
            current_contour: vec![self.params.position],
            fill_color,
        });
    }

    /// Record current position if filling and pen is down
    pub fn record_fill_vertex(&mut self) {
        if let Some(ref mut fill_state) = self.filling {
            if self.params.pen_down {
                tracing::trace!(
                    turtle_id = self.turtle_id,
                    x = self.params.position.x,
                    y = self.params.position.y,
                    vertices = fill_state.current_contour.len() + 1,
                    "Adding vertex to current contour"
                );
                fill_state.current_contour.push(self.params.position);
            } else {
                tracing::trace!(turtle_id = self.turtle_id, "Skipping vertex (pen is up)");
            }
        }
    }

    /// Close the current contour and prepare for a new one (called on `pen_up`)
    pub fn close_fill_contour(&mut self) {
        if let Some(ref mut fill_state) = self.filling {
            tracing::debug!(
                turtle_id = self.turtle_id,
                vertices = fill_state.current_contour.len(),
                "close_fill_contour called"
            );
            // Only close if we have vertices in current contour
            if fill_state.current_contour.len() >= 2 {
                tracing::debug!(
                    turtle_id = self.turtle_id,
                    vertices = fill_state.current_contour.len(),
                    first_x = fill_state.current_contour[0].x,
                    first_y = fill_state.current_contour[0].y,
                    last_x = fill_state.current_contour[fill_state.current_contour.len() - 1].x,
                    last_y = fill_state.current_contour[fill_state.current_contour.len() - 1].y,
                    "Closing contour"
                );
                // Move current contour to completed contours
                let contour = std::mem::take(&mut fill_state.current_contour);
                fill_state.contours.push(contour);
                tracing::debug!(
                    turtle_id = self.turtle_id,
                    completed_contours = fill_state.contours.len(),
                    "Contour moved to completed list"
                );
            } else if !fill_state.current_contour.is_empty() {
                tracing::warn!(
                    turtle_id = self.turtle_id,
                    vertices = fill_state.current_contour.len(),
                    "Current contour has insufficient vertices, not closing"
                );
            } else {
                tracing::warn!(
                    turtle_id = self.turtle_id,
                    "Current contour is empty, nothing to close"
                );
            }
        } else {
            tracing::warn!(
                turtle_id = self.turtle_id,
                "close_fill_contour called but no active fill state"
            );
        }
    }

    /// Start a new contour (called on `pen_down`)
    pub fn start_fill_contour(&mut self) {
        if let Some(ref mut fill_state) = self.filling {
            // Start new contour at current position
            tracing::debug!(
                x = self.params.position.x,
                y = self.params.position.y,
                completed_contours = fill_state.contours.len(),
                self.turtle_id = self.turtle_id,
                "Starting new contour"
            );
            fill_state.current_contour = vec![self.params.position];
        }
    }

    /// Record multiple vertices along a circle arc for filling
    /// This ensures circles are properly filled by sampling points along the arc
    pub fn record_fill_vertices_for_arc(
        &mut self,
        center: Coordinate,
        radius: f32,
        start_angle: f32,
        angle_traveled: f32,
        direction: crate::circle_geometry::CircleDirection,
        steps: u32,
    ) {
        if let Some(ref mut fill_state) = self.filling {
            if self.params.pen_down {
                // Sample points along the arc based on steps
                let num_samples = steps.max(1);

                tracing::trace!(
                    turtle_id = self.turtle_id,
                    center_x = center.x,
                    center_y = center.y,
                    radius = radius,
                    steps = steps,
                    num_samples = num_samples,
                    "Recording arc vertices"
                );

                for i in 1..=num_samples {
                    let progress = i as f32 / num_samples as f32;
                    let current_angle = match direction {
                        crate::circle_geometry::CircleDirection::Left => {
                            start_angle - angle_traveled * progress
                        }
                        crate::circle_geometry::CircleDirection::Right => {
                            start_angle + angle_traveled * progress
                        }
                    };

                    let vertex = Coordinate::new(
                        center.x + radius * current_angle.cos(),
                        center.y + radius * current_angle.sin(),
                    );
                    tracing::trace!(
                        turtle_id = self.turtle_id,
                        vertex_idx = i,
                        x = vertex.x,
                        y = vertex.y,
                        angle_degrees = current_angle.to_degrees(),
                        "Arc vertex"
                    );
                    fill_state.current_contour.push(vertex);
                }
            }
        }
    }

    /// Clear fill state (called after `end_fill`)
    pub fn reset_fill(&mut self) {
        self.filling = None;
    }
}

/// Cached mesh data that can be cloned and converted to Mesh when needed
#[derive(Clone, Debug)]
pub struct MeshData {
    pub vertices: Vec<macroquad::prelude::Vertex>,
    pub indices: Vec<u16>,
}

impl MeshData {
    #[must_use]
    pub fn to_mesh(&self) -> macroquad::prelude::Mesh {
        macroquad::prelude::Mesh {
            vertices: self.vertices.clone(),
            indices: self.indices.clone(),
            texture: None,
        }
    }
}

/// Drawable elements in the world
/// All drawing is done via Lyon-tessellated meshes for consistency and quality
#[derive(Clone, Debug)]
pub struct TurtleSource {
    pub command: crate::commands::TurtleCommand,
    pub color: Color,
    pub fill_color: Color,
    pub pen_width: f32,
    pub start_position: Vec2,
    pub end_position: Vec2,
    pub start_heading: f32,
    pub contours: Option<Vec<Vec<crate::general::Coordinate>>>,
}

#[derive(Clone, Debug)]
pub enum DrawCommand {
    /// Pre-tessellated mesh data (lines, arcs, circles, polygons - all use this)
    Mesh {
        data: MeshData,
        source: TurtleSource,
    },
    /// Text rendering command
    Text {
        text: String,
        position: Vec2,
        heading: f32,
        font_size: crate::general::FontSize,
        color: Color,
        source: TurtleSource,
    },
}

/// The complete turtle world containing all drawing state
pub struct TurtleWorld {
    /// All turtles in the world (indexed by turtle ID)
    pub turtles: Vec<Turtle>,
    pub camera: Camera2D,
    pub background_color: Color,
}

impl TurtleWorld {
    #[must_use]
    pub fn new() -> Self {
        Self {
            turtles: vec![], // Start with no turtles
            camera: Camera2D {
                zoom: vec2(1.0 / screen_width() * 2.0, 1.0 / screen_height() * 2.0),
                target: vec2(0.0, 0.0),
                ..Default::default()
            },
            background_color: WHITE,
        }
    }

    /// Add a new turtle and return its ID
    pub fn add_turtle(&mut self) -> usize {
        let turtle_id = self.turtles.len();
        let new_turtle = Turtle {
            turtle_id,
            ..Default::default()
        };
        self.turtles.push(new_turtle);
        turtle_id
    }

    /// Get turtle by ID
    #[must_use]
    pub fn get_turtle(&self, id: usize) -> Option<&Turtle> {
        self.turtles.get(id)
    }

    /// Get mutable turtle by ID
    pub fn get_turtle_mut(&mut self, id: usize) -> Option<&mut Turtle> {
        self.turtles.get_mut(id)
    }

    /// Reset a specific turtle to default state and remove all its drawings
    pub fn reset_turtle(&mut self, turtle_id: usize) {
        if let Some(turtle) = self.get_turtle_mut(turtle_id) {
            turtle.reset();
            turtle.turtle_id = turtle_id; // Preserve turtle_id after reset
        }
    }

    /// Clear all drawings and reset all turtle states
    pub fn clear(&mut self) {
        for (id, turtle) in self.turtles.iter_mut().enumerate() {
            turtle.reset();
            turtle.turtle_id = id; // Preserve turtle_id after reset
        }
    }
}

impl Default for TurtleWorld {
    fn default() -> Self {
        Self::new()
    }
}
