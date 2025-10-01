use bevy::prelude::{Quat, Transform, Vec2};
use bevy_prototype_lyon::{path::ShapePath, prelude::*, self as lyon_crate};
use lyon_crate::entity::Shape;
use bevy_tweening::Lens;

use crate::general::{angle::Angle, Precision};

pub(crate) struct CircleAnimationLens {
    pub start_pos: Vec2,
    pub center: Vec2,
    pub radii: Vec2,
    pub start: Angle<Precision>,
    pub end: Angle<Precision>,
}

impl Lens<Shape> for CircleAnimationLens {
    fn lerp(&mut self, target: &mut Shape, ratio: f32) {
    let mut path = ShapePath::new();
    path = path.move_to(self.start_pos);
        // The center point of the radius, then the radii in x and y direction, then the angle that will be drawn, then the x_rotation ?
        path = path.arc(
            self.center,
            self.radii,
            (self.start + ((self.end - self.start) * ratio))
                .to_radians()
                .value(),
            0.,
        );
        *target = Shape::from(path);
    }
}

pub(crate) struct CircleMovementLens {
    pub center: Vec2,
    pub start: Transform,
    pub end: Angle<Precision>,
}

impl Lens<Transform> for CircleMovementLens {
    fn lerp(&mut self, target: &mut Transform, ratio: f32) {
        let angle = self.end * ratio;
        let mut rotated = self.start;

        rotated.rotate_around(
            self.center.extend(0.),
            Quat::from_rotation_z(angle.to_radians().value()),
        );

        *target = rotated;
    }
}
