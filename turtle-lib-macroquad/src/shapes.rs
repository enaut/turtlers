//! Turtle shape definitions

use macroquad::prelude::*;
use std::f32::consts::PI;

/// A shape that can be drawn for the turtle
#[derive(Clone, Debug)]
pub struct TurtleShape {
    /// Vertices of the shape (relative to turtle position)
    pub vertices: Vec<Vec2>,
    /// Whether to draw as filled polygon (true) or outline (false)
    pub filled: bool,
}

impl TurtleShape {
    /// Create a new custom shape from vertices
    #[must_use]
    pub fn new(vertices: Vec<Vec2>, filled: bool) -> Self {
        Self { vertices, filled }
    }

    /// Get vertices rotated by the given angle
    #[must_use]
    pub fn rotated_vertices(&self, angle: f32) -> Vec<Vec2> {
        self.vertices
            .iter()
            .map(|v| {
                let cos_a = angle.cos();
                let sin_a = angle.sin();
                vec2(v.x * cos_a - v.y * sin_a, v.x * sin_a + v.y * cos_a)
            })
            .collect()
    }

    /// Triangle shape (simple arrow pointing right)
    #[must_use]
    pub fn triangle() -> Self {
        Self {
            vertices: vec![
                vec2(15.0, 0.0),   // Point
                vec2(-10.0, -8.0), // Bottom left
                vec2(-10.0, 8.0),  // Top left
            ],
            filled: true,
        }
    }

    /// Classic turtle shape
    #[must_use]
    pub fn turtle() -> Self {
        // Based on the original turtle shape from turtle-lib
        let polygon: &[[f32; 2]; 23] = &[
            [-2.5, 14.0],
            [-1.25, 10.0],
            [-4.0, 7.0],
            [-7.0, 9.0],
            [-9.0, 8.0],
            [-6.0, 5.0],
            [-7.0, 1.0],
            [-5.0, -3.0],
            [-8.0, -6.0],
            [-6.0, -8.0],
            [-4.0, -5.0],
            [0.0, -7.0],
            [4.0, -5.0],
            [6.0, -8.0],
            [8.0, -6.0],
            [5.0, -3.0],
            [7.0, 1.0],
            [6.0, 5.0],
            [9.0, 8.0],
            [7.0, 9.0],
            [4.0, 7.0],
            [1.25, 10.0],
            [2.5, 14.0],
        ];

        // Rotate by -90 degrees to point right (original points up)
        let vertices: Vec<Vec2> = polygon
            .iter()
            .map(|[x, y]| {
                let v = vec2(*x, *y);
                let cos_a = (-PI / 2.0).cos();
                let sin_a = (-PI / 2.0).sin();
                vec2(v.x * cos_a - v.y * sin_a, v.x * sin_a + v.y * cos_a)
            })
            .collect();

        Self {
            vertices,
            filled: true, // Now uses ear clipping for proper concave polygon rendering
        }
    }

    /// Circle shape
    #[must_use]
    pub fn circle() -> Self {
        let segments = 16;
        let radius = 10.0;
        let vertices: Vec<Vec2> = (0..segments)
            .map(|i| {
                let angle = (i as f32 / segments as f32) * 2.0 * PI;
                vec2(radius * angle.cos(), radius * angle.sin())
            })
            .collect();

        Self {
            vertices,
            filled: true,
        }
    }

    /// Square shape
    #[must_use]
    pub fn square() -> Self {
        Self {
            vertices: vec![
                vec2(8.0, 8.0),
                vec2(-8.0, 8.0),
                vec2(-8.0, -8.0),
                vec2(8.0, -8.0),
            ],
            filled: true,
        }
    }

    /// Arrow shape (simple arrow pointing right)
    #[must_use]
    pub fn arrow() -> Self {
        Self {
            vertices: vec![
                vec2(12.0, 0.0),  // Point
                vec2(-8.0, 6.0),  // Top back
                vec2(-4.0, 0.0),  // Middle back
                vec2(-8.0, -6.0), // Bottom back
            ],
            filled: true,
        }
    }
}

/// Pre-defined shape types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ShapeType {
    Triangle,
    #[default]
    Turtle,
    Circle,
    Square,
    Arrow,
}

impl ShapeType {
    /// Get the corresponding `TurtleShape`
    #[must_use]
    pub fn to_shape(&self) -> TurtleShape {
        match self {
            ShapeType::Triangle => TurtleShape::triangle(),
            ShapeType::Turtle => TurtleShape::turtle(),
            ShapeType::Circle => TurtleShape::circle(),
            ShapeType::Square => TurtleShape::square(),
            ShapeType::Arrow => TurtleShape::arrow(),
        }
    }
}
