//! Turtle state and world state management

use crate::general::{Angle, AnimationSpeed, Color, Coordinate, Precision};
use crate::shapes::TurtleShape;
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

/// State of a single turtle
#[derive(Clone, Debug)]
pub struct TurtleState {
    pub position: Coordinate,
    pub heading: Precision, // radians
    pub pen_down: bool,
    pub color: Color,
    pub fill_color: Option<Color>,
    pub pen_width: Precision,
    pub speed: AnimationSpeed,
    pub visible: bool,
    pub shape: TurtleShape,

    // Fill tracking
    pub filling: Option<FillState>,
}

impl Default for TurtleState {
    fn default() -> Self {
        Self {
            position: vec2(0.0, 0.0),
            heading: 0.0, // pointing right (0 radians)
            pen_down: true,
            color: BLACK,
            fill_color: None,
            pen_width: 2.0,
            speed: AnimationSpeed::default(),
            visible: true,
            shape: TurtleShape::turtle(),
            filling: None,
        }
    }
}

impl TurtleState {
    pub fn set_speed(&mut self, speed: AnimationSpeed) {
        self.speed = speed;
    }

    #[must_use]
    pub fn heading_angle(&self) -> Angle {
        Angle::radians(self.heading)
    }

    /// Reset turtle to default state
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Start recording fill vertices
    pub fn begin_fill(&mut self, fill_color: Color) {
        self.filling = Some(FillState {
            start_position: self.position,
            contours: Vec::new(),
            current_contour: vec![self.position],
            fill_color,
        });
    }

    /// Record current position if filling and pen is down
    pub fn record_fill_vertex(&mut self) {
        if let Some(ref mut fill_state) = self.filling {
            if self.pen_down {
                tracing::trace!(
                    x = self.position.x,
                    y = self.position.y,
                    vertices = fill_state.current_contour.len() + 1,
                    "Adding vertex to current contour"
                );
                fill_state.current_contour.push(self.position);
            } else {
                tracing::trace!("Skipping vertex (pen is up)");
            }
        }
    }

    /// Close the current contour and prepare for a new one (called on `pen_up`)
    pub fn close_fill_contour(&mut self) {
        if let Some(ref mut fill_state) = self.filling {
            tracing::debug!(
                vertices = fill_state.current_contour.len(),
                "close_fill_contour called"
            );
            // Only close if we have vertices in current contour
            if fill_state.current_contour.len() >= 2 {
                tracing::debug!(
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
                    completed_contours = fill_state.contours.len(),
                    "Contour moved to completed list"
                );
            } else if !fill_state.current_contour.is_empty() {
                tracing::warn!(
                    vertices = fill_state.current_contour.len(),
                    "Current contour has insufficient vertices, not closing"
                );
            } else {
                tracing::warn!("Current contour is empty, nothing to close");
            }
        } else {
            tracing::warn!("close_fill_contour called but no active fill state");
        }
    }

    /// Start a new contour (called on `pen_down`)
    pub fn start_fill_contour(&mut self) {
        if let Some(ref mut fill_state) = self.filling {
            // Start new contour at current position
            tracing::debug!(
                x = self.position.x,
                y = self.position.y,
                completed_contours = fill_state.contours.len(),
                "Starting new contour"
            );
            fill_state.current_contour = vec![self.position];
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
            if self.pen_down {
                // Sample points along the arc based on steps
                let num_samples = steps as usize;

                tracing::trace!(
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
pub enum DrawCommand {
    /// Pre-tessellated mesh data (lines, arcs, circles, polygons - all use this)
    /// Includes the turtle ID that created this command
    Mesh { turtle_id: usize, data: MeshData },
}

/// The complete turtle world containing all drawing state
pub struct TurtleWorld {
    /// All turtles in the world (indexed by turtle ID)
    pub turtles: Vec<TurtleState>,
    /// All drawing commands from all turtles
    pub commands: Vec<DrawCommand>,
    pub camera: Camera2D,
    pub background_color: Color,
}

impl TurtleWorld {
    #[must_use]
    pub fn new() -> Self {
        Self {
            turtles: vec![TurtleState::default()], // Start with one default turtle
            commands: Vec::new(),
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
        self.turtles.push(TurtleState::default());
        self.turtles.len() - 1
    }

    /// Get turtle by ID
    #[must_use]
    pub fn get_turtle(&self, id: usize) -> Option<&TurtleState> {
        self.turtles.get(id)
    }

    /// Get mutable turtle by ID
    pub fn get_turtle_mut(&mut self, id: usize) -> Option<&mut TurtleState> {
        self.turtles.get_mut(id)
    }

    /// Reset a specific turtle to default state and remove all its drawings
    pub fn reset_turtle(&mut self, turtle_id: usize) {
        if let Some(turtle) = self.get_turtle_mut(turtle_id) {
            turtle.reset();
        }
        // Remove all commands created by this turtle
        self.commands.retain(|cmd| match cmd {
            DrawCommand::Mesh { turtle_id: id, .. } => *id != turtle_id,
        });
    }

    pub fn add_command(&mut self, cmd: DrawCommand) {
        self.commands.push(cmd);
    }

    /// Clear all drawings and reset all turtle states
    pub fn clear(&mut self) {
        self.commands.clear();
        for turtle in &mut self.turtles {
            turtle.reset();
        }
    }
}

impl Default for TurtleWorld {
    fn default() -> Self {
        Self::new()
    }
}
