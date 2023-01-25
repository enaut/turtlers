use bevy::{
    prelude::{Bundle, Color, Component, Name, Transform, Vec2},
    reflect::{FromReflect, Reflect},
};
use bevy_prototype_lyon::{
    entity::ShapeBundle,
    prelude::{DrawMode, FillMode, GeometryBuilder, PathBuilder, StrokeMode},
    shapes::Line,
};

use crate::general::{angle::Angle, Precision};

#[derive(Bundle, Reflect, FromReflect, Default)]
pub struct TurtleDrawLine {
    #[reflect(ignore)]
    line: ShapeBundle,
    name: Name,
    marker: LineMarker,
}

impl std::fmt::Debug for TurtleDrawLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TurtleDrawLine")
            .field("name", &self.name)
            .field("marker", &self.marker)
            .finish()
    }
}

#[derive(Component, Default, Reflect, FromReflect, Debug, Clone, Copy)]
struct LineMarker;

impl TurtleDrawLine {
    pub(crate) fn new(start: Vec2, end: Vec2) -> Self {
        Self {
            line: GeometryBuilder::build_as(
                &Line(start, start),
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::MIDNIGHT_BLUE),
                    outline_mode: StrokeMode::new(Color::BLACK, 1.0),
                },
                Transform::IDENTITY,
            ),
            name: Name::new(format!("Line {}-{}", start, end)),
            marker: LineMarker,
        }
    }
}

#[derive(Bundle, Reflect, FromReflect, Default)]

pub struct TurtleDrawCircle {
    #[reflect(ignore)]
    line: ShapeBundle,
    name: Name,
    marker: CircleMarker,
}

impl std::fmt::Debug for TurtleDrawCircle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TurtleDrawCircle")
            .field("name", &self.name)
            .field("marker", &self.marker)
            .finish()
    }
}

#[derive(Component, Default, Reflect, FromReflect, Debug, Clone)]
struct CircleMarker;

impl TurtleDrawCircle {
    pub(crate) fn new(
        center: Vec2,
        radii: Vec2,
        angle: Angle<Precision>,
        start: Vec2,
        _end: Vec2,
    ) -> Self {
        let mut path_builder = PathBuilder::new();
        path_builder.move_to(start);
        // The center point of the radius - this is responsible for the orientation of the ellipse,
        // then the radii in x and y direction - this can be rotated using the x_rotation parameter,
        // then the angle - the part of the circle that will be drawn like (PI/2.0) for a quarter circle,
        // then the x_rotation (maybe the rotation of the radii?)
        path_builder.arc(center, radii, angle.to_radians().value(), 0.);
        let line = path_builder.build();
        println!("Draw Circle: {} {} {:?}", center, radii, angle);

        Self {
            line: GeometryBuilder::build_as(
                &line,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::rgba(0., 0., 0., 0.)),
                    outline_mode: StrokeMode::new(Color::BLACK, 1.0),
                },
                Transform::IDENTITY,
            ),
            name: Name::new(format!("Circle at {}, {}", center.x, center.y)),
            marker: CircleMarker,
        }
    }
}
