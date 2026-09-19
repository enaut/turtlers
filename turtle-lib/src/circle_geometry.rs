//! Circle geometry calculations - single source of truth for `circle_left` and `circle_right`

use crate::general::Radians;
use macroquad::prelude::*;

/// Generate evenly-spaced points along a circular arc.
///
/// Returns exactly `steps` points, uniformly distributed from (not including)
/// the arc start to (including) the arc end.  This is the **single source of
/// truth** for arc sampling used by tessellation, tween stroke drawing, and
/// fill-polygon preview.
///
/// # Arguments
/// * `center`      — centre of the circle
/// * `radius`      — arc radius
/// * `start_angle` — angle from `center` to the turtle's start position (radians)
/// * `sweep_angle` — total arc sweep in radians (absolute; sign comes from `direction`)
/// * `steps`       — number of sample points (clamped to ≥ 1)
/// * `direction`   — which way the arc curves
pub(crate) fn arc_points(
    center: Vec2,
    radius: f32,
    start_angle: f32,
    sweep_angle: f32,
    steps: usize,
    direction: CircleDirection,
) -> Vec<Vec2> {
    let n = steps.max(1);
    let step_size = sweep_angle / n as f32;
    (1..=n)
        .map(|i| {
            let a = match direction {
                CircleDirection::Left => start_angle - step_size * i as f32,
                CircleDirection::Right => start_angle + step_size * i as f32,
            };
            Vec2::new(center.x + radius * a.cos(), center.y + radius * a.sin())
        })
        .collect()
}

/// Direction of circular motion (in screen coordinates with Y-down)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircleDirection {
    Left,  // Counter-clockwise visually, heading decreases
    Right, // Clockwise visually, heading increases
}

/// Encapsulates all geometry for a circular arc
pub(crate) struct CircleGeometry {
    pub(crate) center: Vec2,
    pub(crate) radius: f32,
    pub(crate) start_angle_from_center: f32, // radians
    pub(crate) direction: CircleDirection,
}

impl CircleGeometry {
    /// Create geometry for a circle command
    #[must_use]
    pub fn new(
        turtle_pos: Vec2,
        turtle_heading: Radians,
        radius: f32,
        direction: CircleDirection,
    ) -> Self {
        use std::f32::consts::FRAC_PI_2;

        // Extract raw f32 once — all arithmetic below is in radians
        let heading = turtle_heading.value();

        // Calculate center based on direction
        // In screen coordinates (Y-down):
        // - Left turn (counter-clockwise visually): center is perpendicular-left from turtle's perspective
        //   which is heading - π/2 (rotated clockwise from heading vector)
        // - Right turn (clockwise visually): center is perpendicular-right from turtle's perspective
        //   which is heading + π/2 (rotated counter-clockwise from heading vector)
        let center_offset_angle = match direction {
            CircleDirection::Left => heading - FRAC_PI_2,
            CircleDirection::Right => heading + FRAC_PI_2,
        };

        let center = vec2(
            turtle_pos.x + radius * center_offset_angle.cos(),
            turtle_pos.y + radius * center_offset_angle.sin(),
        );

        // Angle from center back to turtle position
        let start_angle_from_center = match direction {
            CircleDirection::Left => heading + FRAC_PI_2,
            CircleDirection::Right => heading - FRAC_PI_2,
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


}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::{FRAC_PI_2, PI};

    #[test]
    fn test_circle_left_geometry() {
        let geom = CircleGeometry::new(
            vec2(0.0, 0.0),
            Radians::new(0.0), // heading east (0 radians)
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
            Radians::new(0.0), // heading east
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
