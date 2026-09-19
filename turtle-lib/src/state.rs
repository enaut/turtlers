//! Turtle state and world state management

use crate::commands::CommandQueue;
use crate::general::{AnimationSpeed, Color, Coordinate};
use crate::shapes::TurtleShape;
use crate::tweening::TweenController;
use macroquad::prelude::*;

/// State during active fill operation
#[derive(Clone, Debug)]
pub(crate) struct FillState {
    /// All contours collected so far. Each contour is a separate closed path.
    /// The first contour is the outer boundary, subsequent contours are holes.
    pub(crate) contours: Vec<Vec<Coordinate>>,

    /// Current contour being built (vertices for the active `pen_down` segment)
    pub(crate) current_contour: Vec<Coordinate>,

    /// Fill color (cached from when `begin_fill` was called)
    pub(crate) fill_color: Color,
}

/// Parameters that define a turtle's visual state
#[derive(Clone, Debug)]
pub(crate) struct TurtleParams {
    pub(crate) position: Vec2,
    pub(crate) heading: f32,
    pub(crate) pen_down: bool,
    pub(crate) pen_width: f32,
    pub(crate) color: Color,
    pub(crate) fill_color: Option<Color>,
    pub(crate) visible: bool,
    pub(crate) shape: crate::shapes::TurtleShape,
    pub(crate) speed: AnimationSpeed,
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
pub(crate) struct Turtle {
    #[allow(clippy::struct_field_names)]
    pub(crate) turtle_id: usize,
    pub(crate) params: TurtleParams,

    // Fill tracking
    pub(crate) filling: Option<FillState>,

    // Drawing commands created by this turtle
    pub(crate) commands: Vec<DrawCommand>,

    // SVG draw-event log — populated alongside `commands`, consumed by the SVG exporter
    pub(crate) svg_log: SvgLog,

    // Animation controller for this turtle
    pub(crate) tween_controller: TweenController,
}

impl Default for Turtle {
    fn default() -> Self {
        Self {
            turtle_id: 0,
            params: TurtleParams::default(),
            filling: None,
            commands: Vec::new(),
            svg_log: SvgLog::default(),
            tween_controller: TweenController::new(CommandQueue::new(), AnimationSpeed::default()),
        }
    }
}

impl Turtle {
    pub fn set_speed(&mut self, speed: AnimationSpeed) {
        self.params.speed = speed;
    }

    /// Drive the animation controller for one frame.
    ///
    /// Returns `(command, start_params, end_params)` for every command that
    /// completed this frame and whose stroke needs to be tessellated by the
    /// caller into a `DrawCommand`.
    ///
    /// This method performs the correct disjoint field-borrow split so that
    /// `TweenController::update` can be a proper `&mut self` method instead
    /// of the old static-method borrow-checker workaround.
    pub fn update_tweens(
        &mut self,
    ) -> Vec<(crate::commands::TurtleCommand, TurtleParams, TurtleParams)> {
        self.tween_controller.update(
            self.turtle_id,
            &mut self.params,
            &mut self.filling,
            &mut self.commands,
            &mut self.svg_log,
        )
    }
}

/// The draw-event log for SVG export.
///
/// When the `svg` feature is **disabled** this is a zero-sized type (ZST) with
/// no fields — it compiles away entirely and adds zero overhead to `Turtle`.
/// When the feature is **enabled** it owns a `Vec<SvgRecord>` that the SVG
/// exporter consumes after rendering.
///
/// All methods are always callable so function signatures that accept
/// `&mut SvgLog` need no feature-gating at the parameter level.
#[derive(Clone, Debug, Default)]
pub(crate) struct SvgLog {
    #[cfg(feature = "svg")]
    pub(crate) records: Vec<SvgRecord>,
}

impl SvgLog {
    pub(crate) fn clear(&mut self) {
        #[cfg(feature = "svg")]
        self.records.clear();
    }

    #[cfg(feature = "svg")]
    pub(crate) fn push(&mut self, record: SvgRecord) {
        self.records.push(record);
    }
}

/// A drawing event captured for SVG export.
///
/// Only compiled when the `svg` feature is enabled.
#[cfg(feature = "svg")]
#[derive(Clone, Debug)]
pub(crate) enum SvgRecord {
    /// A straight-line stroke.
    Line {
        start: Vec2,
        end: Vec2,
        color: Color,
        pen_width: f32,
    },
    /// An arc or full-circle stroke.
    Arc {
        start_position: Vec2,
        start_heading: f32,
        radius: crate::general::Precision,
        angle: crate::general::Degrees,
        direction: crate::circle_geometry::CircleDirection,
        color: Color,
        pen_width: f32,
    },
    /// A filled region (potentially with holes via the even-odd rule).
    Fill {
        contours: Vec<Vec<crate::general::Coordinate>>,
        fill_color: Color,
        stroke_color: Color,
    },
    /// A text element.
    Text {
        text: String,
        position: Vec2,
        heading: f32,
        font_size: crate::general::FontSize,
        color: Color,
    },
}

/// Drawable elements in the world.
/// All drawing is done via Lyon-tessellated meshes for consistency and quality.
pub(crate) enum DrawCommand {
    /// Pre-tessellated mesh data (lines, arcs, circles, polygons — all use this).
    Mesh(macroquad::prelude::Mesh),
    /// Text rendering command.
    Text {
        text: String,
        position: Vec2,
        heading: f32,
        font_size: crate::general::FontSize,
        color: Color,
    },
}

/// The complete turtle world containing all drawing state
pub(crate) struct TurtleWorld {
    /// All turtles in the world (indexed by turtle ID)
    pub(crate) turtles: Vec<Turtle>,
    pub(crate) camera: Camera2D,
}

impl TurtleWorld {
    #[must_use]
    pub fn new() -> Self {
        Self {
            turtles: vec![], // Start with no turtles
            camera: Camera2D::default(),
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

    /// Get mutable turtle by ID
    pub fn get_turtle_mut(&mut self, id: usize) -> Option<&mut Turtle> {
        self.turtles.get_mut(id)
    }
}

impl Default for TurtleWorld {
    fn default() -> Self {
        Self::new()
    }
}
