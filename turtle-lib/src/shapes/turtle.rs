use std::f32::consts::PI;

use bevy::prelude::Vec2;
use bevy_prototype_lyon::prelude::{Path, PathBuilder};

use crate::general::Precision;

pub fn turtle() -> Path {
    let polygon: &[[Precision; 2]; 23] = &[
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
    let mut turtle_path = PathBuilder::new();
    turtle_path.line_to(Vec2::new(1.0, 1.0));
    turtle_path.line_to(Vec2::new(-1.0, 1.0));
    turtle_path.line_to(Vec2::new(-1.0, -1.0));
    turtle_path.line_to(Vec2::new(1.0, -1.0));
    turtle_path.close();
    turtle_path.move_to(Vec2::new(0.0, 16.0).rotate(Vec2::from_angle(-PI / 2.)));
    for coord in polygon {
        turtle_path.line_to(Vec2::from_array(*coord).rotate(Vec2::from_angle(-PI / 2.)));
    }
    turtle_path.close();
    turtle_path.build()
}
