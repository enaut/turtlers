//! Circle geometry calculations - single source of truth for `circle_left` and `circle_right`

use macroquad::prelude::*;

/// Direction of circular motion (in screen coordinates with Y-down)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircleDirection {
    Left,  // Counter-clockwise visually, heading decreases
    Right, // Clockwise visually, heading increases
}

/// Encapsulates all geometry for a circular arc
pub struct CircleGeometry {
    pub center: Vec2,
    pub radius: f32,
    pub start_angle_from_center: f32, // radians
    pub direction: CircleDirection,
}

impl CircleGeometry {
    /// Create geometry for a circle command
    #[must_use]
    pub fn new(
        turtle_pos: Vec2,
        turtle_heading: f32,
        radius: f32,
        direction: CircleDirection,
    ) -> Self {
        use std::f32::consts::FRAC_PI_2;

        // Calculate center based on direction
        // In screen coordinates (Y-down):
        // - Left turn (counter-clockwise visually): center is perpendicular-left from turtle's perspective
        //   which is heading - π/2 (rotated clockwise from heading vector)
        // - Right turn (clockwise visually): center is perpendicular-right from turtle's perspective
        //   which is heading + π/2 (rotated counter-clockwise from heading vector)
        let center_offset_angle = match direction {
            CircleDirection::Left => turtle_heading - FRAC_PI_2,
            CircleDirection::Right => turtle_heading + FRAC_PI_2,
        };

        let center = vec2(
            turtle_pos.x + radius * center_offset_angle.cos(),
            turtle_pos.y + radius * center_offset_angle.sin(),
        );

        // Angle from center back to turtle position
        let start_angle_from_center = match direction {
            CircleDirection::Left => turtle_heading + FRAC_PI_2,
            CircleDirection::Right => turtle_heading - FRAC_PI_2,
        };

        Self {
            center,
            radius,
            start_angle_from_center,
            direction,
        }
    }

    /// Calculate position after traveling an angle along the arc
    #[must_use]
    pub fn position_at_angle(&self, angle_traveled: f32) -> Vec2 {
        let current_angle = match self.direction {
            CircleDirection::Left => self.start_angle_from_center - angle_traveled,
            CircleDirection::Right => self.start_angle_from_center + angle_traveled,
        };

        vec2(
            self.center.x + self.radius * current_angle.cos(),
            self.center.y + self.radius * current_angle.sin(),
        )
    }

    /// Calculate position at a given progress (0.0 to 1.0) through `total_angle`
    #[must_use]
    pub fn position_at_progress(&self, total_angle: f32, progress: f32) -> Vec2 {
        let angle_traveled = total_angle * progress;
        self.position_at_angle(angle_traveled)
    }

    /// Get the angle traveled from start position to a given position
    #[must_use]
    pub fn angle_to_position(&self, position: Vec2) -> f32 {
        let displacement = position - self.center;
        let current_angle = displacement.y.atan2(displacement.x);

        let mut angle_diff = match self.direction {
            CircleDirection::Left => self.start_angle_from_center - current_angle,
            CircleDirection::Right => current_angle - self.start_angle_from_center,
        };

        // Normalize to [0, 2π)
        if angle_diff < 0.0 {
            angle_diff += 2.0 * std::f32::consts::PI;
        }

        angle_diff
    }

    /// Get `draw_arc` parameters for the full arc
    /// Returns (`rotation_degrees`, `arc_degrees`) for macroquad's `draw_arc`
    #[must_use]
    pub fn draw_arc_params(&self, total_angle_degrees: f32) -> (f32, f32) {
        match self.direction {
            CircleDirection::Left => {
                // For left (counter-clockwise), we need to draw counter-clockwise from end back to start
                // so we start at (start - total_angle) and draw total_angle counter-clockwise
                let end_angle = self.start_angle_from_center - total_angle_degrees.to_radians();
                (end_angle.to_degrees(), total_angle_degrees)
            }
            CircleDirection::Right => {
                // For right (clockwise), draw from start
                (
                    self.start_angle_from_center.to_degrees(),
                    total_angle_degrees,
                )
            }
        }
    }

    /// Get `draw_arc` parameters for a partial arc (during tweening)
    /// Returns (`rotation_degrees`, `arc_degrees`) for macroquad's `draw_arc`
    #[must_use]
    pub fn draw_arc_params_partial(&self, angle_traveled: f32) -> (f32, f32) {
        let angle_traveled_degrees = angle_traveled.to_degrees();

        match self.direction {
            CircleDirection::Left => {
                // Draw from current position backwards (counter-clockwise) to start
                let current_angle = self.start_angle_from_center - angle_traveled;
                (current_angle.to_degrees(), angle_traveled_degrees)
            }
            CircleDirection::Right => {
                // Draw from start, counter-clockwise
                (
                    self.start_angle_from_center.to_degrees(),
                    angle_traveled_degrees,
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::{FRAC_PI_2, PI};

    #[test]
    fn test_circle_left_geometry() {
        let geom = CircleGeometry::new(
            vec2(0.0, 0.0),
            0.0, // heading east (0 radians)
            100.0,
            CircleDirection::Left,
        );

        // For left turn with heading east (0), center should be at heading - π/2
        // That's -π/2 radians = south
        // Center = start + 100 * (cos(-π/2), sin(-π/2)) = (0, 0) + (0, -100) = (0, -100)
        assert!(
            (geom.center.x - 0.0).abs() < 0.01,
            "center.x = {}",
            geom.center.x
        );
        assert!(
            (geom.center.y - (-100.0)).abs() < 0.01,
            "center.y = {}",
            geom.center.y
        );

        // After π/2 radians counter-clockwise around a circle centered at (0, -100):
        // start_angle = π/2 (pointing north from center, which is where (0,0) is)
        // after π/2 counter-clockwise (subtract in screen coords): angle = π/2 - π/2 = 0 (pointing east from center)
        // pos = (0, -100) + 100 * (cos(0), sin(0)) = (0, -100) + (100, 0) = (100, -100)
        let pos = geom.position_at_angle(FRAC_PI_2);
        assert!((pos.x - 100.0).abs() < 0.01, "pos.x = {}", pos.x);
        assert!((pos.y - (-100.0)).abs() < 0.01, "pos.y = {}", pos.y);
    }

    #[test]
    fn test_circle_right_geometry() {
        let geom = CircleGeometry::new(
            vec2(0.0, 0.0),
            0.0, // heading east
            100.0,
            CircleDirection::Right,
        );

        // For right turn with heading east (0), center should be at heading + π/2
        // That's π/2 radians = north
        // Center = start + 100 * (cos(π/2), sin(π/2)) = (0, 0) + (0, 100) = (0, 100)
        assert!(
            (geom.center.x - 0.0).abs() < 0.01,
            "center.x = {}",
            geom.center.x
        );
        assert!(
            (geom.center.y - 100.0).abs() < 0.01,
            "center.y = {}",
            geom.center.y
        );

        // After π/2 radians clockwise around a circle centered at (0, 100):
        // start_angle = -π/2 (pointing south from center, which is where (0,0) is)
        // after π/2 clockwise (add in screen coords): angle = -π/2 + π/2 = 0 (pointing east from center)
        // pos = (0, 100) + 100 * (cos(0), sin(0)) = (0, 100) + (100, 0) = (100, 100)
        let pos = geom.position_at_angle(PI / 2.0);
        assert!((pos.x - 100.0).abs() < 0.01, "pos.x = {}", pos.x);
        assert!((pos.y - 100.0).abs() < 0.01, "pos.y = {}", pos.y);
    }
}
