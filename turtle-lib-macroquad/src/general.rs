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

/// Execution speed setting
/// - Instant: No animation, commands execute immediately
/// - Animated(speed): Smooth animation at specified pixels/second
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AnimationSpeed {
    Instant,
    Animated(f32), // pixels per second
}

impl AnimationSpeed {
    /// Check if this is instant mode
    pub fn is_instant(&self) -> bool {
        matches!(self, AnimationSpeed::Instant)
    }

    /// Get the speed value (returns a high value for Instant)
    pub fn value(&self) -> f32 {
        match self {
            AnimationSpeed::Instant => 9999.0,
            AnimationSpeed::Animated(speed) => *speed,
        }
    }

    /// Create from a raw speed value (>= 999 becomes Instant)
    pub fn from_value(speed: f32) -> Self {
        if speed >= 999.0 {
            AnimationSpeed::Instant
        } else {
            AnimationSpeed::Animated(speed.max(1.0))
        }
    }

    /// Create from a u32 value for backward compatibility
    pub fn from_u32(speed: u32) -> Self {
        Self::from_value(speed as f32)
    }
}

impl Default for AnimationSpeed {
    fn default() -> Self {
        AnimationSpeed::Animated(100.0)
    }
}

impl From<f32> for AnimationSpeed {
    fn from(speed: f32) -> Self {
        AnimationSpeed::from_value(speed)
    }
}

impl From<u32> for AnimationSpeed {
    fn from(speed: u32) -> Self {
        AnimationSpeed::from_u32(speed)
    }
}

/// Color type re-export from macroquad
pub use macroquad::color::Color;
