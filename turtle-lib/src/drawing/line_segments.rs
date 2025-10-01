use bevy::{
    prelude::{Bundle, Color, Component, Name, Vec2},
    reflect::Reflect,
};
use bevy_prototype_lyon::{
    draw::{Fill, Stroke},
    entity::Shape,
    geometry::ShapeBuilder,
    path::ShapePath,
    prelude::ShapeBuilderBase as _,
    shapes::Line,
};

use crate::general::{angle::Angle, Precision};

#[derive(Bundle, Reflect, Default)]
pub struct TurtleDrawLine {
    #[reflect(ignore)]
    line: Shape,
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

#[derive(Component, Default, Reflect, Debug, Clone, Copy)]
struct LineMarker;

impl TurtleDrawLine {
    pub(crate) fn new(start: Vec2, end: Vec2) -> Self {
        let line = Line(start, end);
        Self {
            line: ShapeBuilder::with(&line)
                .fill(Fill::color(Color::NONE))
                .stroke(Stroke::new(Color::srgb(0.0, 0.0, 0.0), 1.0))
                .build(),
            name: Name::new(format!("Line {}-{}", start, end)),
            marker: LineMarker,
        }
    }
}

#[derive(Bundle, Reflect, Default)]

pub struct TurtleDrawCircle {
    #[reflect(ignore)]
    line: Shape,
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

#[derive(Component, Default, Reflect, Debug, Clone)]
struct CircleMarker;

impl TurtleDrawCircle {
    pub(crate) fn new(
        center: Vec2,
        radii: Vec2,
        angle: Angle<Precision>,
        start: Vec2,
        _end: Vec2,
    ) -> Self {
        // The center point of the radius - this is responsible for the orientation of the ellipse,
        // then the radii in x and y direction - this can be rotated using the x_rotation parameter,
        // then the angle - the part of the circle that will be drawn like (PI/2.0) for a quarter circle,
        // then the x_rotation (maybe the rotation of the radii?)
        let path =
            ShapePath::new()
                .move_to(start)
                .arc(center, radii, angle.to_radians().value(), 0.);

        println!("Draw Circle: {} {} {:?}", center, radii, angle);

        Self {
            line: ShapeBuilder::with(&path)
                .fill(Fill::color(Color::NONE))
                .stroke(Stroke::new(Color::srgb(0.0, 0.0, 0.0), 1.0))
                .build(),
            name: Name::new(format!("Circle at {}, {}", center.x, center.y)),
            marker: CircleMarker,
        }
    }
}
