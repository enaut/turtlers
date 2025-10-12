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
/// - `Instant(draw_calls)`: Fast execution with limited draw calls per frame (speed - 1000, minimum 1)
/// - `Animated(speed)`: Smooth animation at specified pixels/second
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AnimationSpeed {
    Instant(u32),  // Number of draw calls per frame (minimum 1)
    Animated(f32), // pixels per second
}

impl AnimationSpeed {
    /// Check if this is instant mode
    #[must_use]
    pub fn is_animating(&self) -> bool {
        matches!(self, AnimationSpeed::Animated(_))
    }

    /// Get the speed value (returns encoded value for Instant)
    #[must_use]
    pub fn value(&self) -> f32 {
        match self {
            AnimationSpeed::Instant(calls) => 1000.0 + *calls as f32,
            AnimationSpeed::Animated(speed) => *speed,
        }
    }

    /// Create from a raw speed value
    /// - speed >= 1000 becomes Instant with max(1, speed - 1000) draw calls per frame
    /// - speed < 1000 becomes Animated
    #[must_use]
    pub fn from_value(speed: f32) -> Self {
        if speed >= 1000.0 {
            let draw_calls = (speed - 1000.0).max(1.0) as u32; // Ensure at least 1
            AnimationSpeed::Instant(draw_calls)
        } else {
            AnimationSpeed::Animated(speed.max(1.0))
        }
    }

    /// Create from a u32 value for backward compatibility
    #[must_use]
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
