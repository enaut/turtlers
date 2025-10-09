//! General types and type aliases used throughout the turtle library

use macroquad::prelude::*;

pub mod angle;
pub mod length;

pub use angle::Angle;
pub use length::Length;

/// Precision type for calculations
pub type Precision = f32;

/// 2D coordinate in screen space
pub type Coordinate = Vec2;

/// Visibility flag for turtle
pub type Visibility = bool;

/// Speed of animations (higher = faster, >= 999 = instant)
pub type Speed = u32;

/// Color type re-export from macroquad
pub use macroquad::color::Color;
