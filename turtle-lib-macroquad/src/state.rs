//! Turtle state and world state management

use crate::general::{Angle, Color, Coordinate, Precision, Speed};
use crate::shapes::TurtleShape;
use macroquad::prelude::*;

/// State of a single turtle
#[derive(Clone, Debug)]
pub struct TurtleState {
    pub position: Coordinate,
    pub heading: Precision, // radians
    pub pen_down: bool,
    pub color: Color,
    pub fill_color: Option<Color>,
    pub pen_width: Precision,
    pub speed: Speed,
    pub visible: bool,
    pub shape: TurtleShape,
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
            speed: 100, // pixels per second
            visible: true,
            shape: TurtleShape::turtle(),
        }
    }
}

impl TurtleState {
    pub fn set_speed(&mut self, speed: Speed) {
        self.speed = speed.max(1);
    }

    pub fn heading_angle(&self) -> Angle {
        Angle::radians(self.heading)
    }
}

/// Drawable elements in the world
#[derive(Clone, Debug)]
pub enum DrawCommand {
    Line {
        start: Coordinate,
        end: Coordinate,
        color: Color,
        width: Precision,
    },
    Circle {
        center: Coordinate,
        radius: Precision,
        color: Color,
        filled: bool,
    },
    Arc {
        center: Coordinate,
        radius: Precision,
        rotation: Precision, // Start angle in degrees
        arc: Precision,      // Arc extent in degrees
        color: Color,
        width: Precision,
        sides: u8, // Number of segments for quality
    },
    FilledPolygon {
        vertices: Vec<Coordinate>,
        color: Color,
    },
}

/// The complete turtle world containing all drawing state
pub struct TurtleWorld {
    pub turtle: TurtleState,
    pub commands: Vec<DrawCommand>,
    pub camera: Camera2D,
    pub background_color: Color,
}

impl TurtleWorld {
    pub fn new() -> Self {
        Self {
            turtle: TurtleState::default(),
            commands: Vec::new(),
            camera: Camera2D {
                zoom: vec2(1.0 / screen_width() * 2.0, 1.0 / screen_height() * 2.0),
                target: vec2(0.0, 0.0),
                ..Default::default()
            },
            background_color: WHITE,
        }
    }

    pub fn add_command(&mut self, cmd: DrawCommand) {
        self.commands.push(cmd);
    }

    pub fn clear(&mut self) {
        self.commands.clear();
        self.turtle = TurtleState::default();
    }
}

impl Default for TurtleWorld {
    fn default() -> Self {
        Self::new()
    }
}
