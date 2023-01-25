use std::{cmp::max, time::Duration};

use bevy::{
    prelude::{Component, Transform},
    reflect::Reflect,
};

use crate::{
    commands::TurtleSegment,
    general::{angle::Angle, Coordinate, Precision, Speed, Visibility},
    shapes::TurtleColors,
};

/// Describing the full state of a turtle.
#[derive(Component, Reflect, Default, Debug, Clone)]
pub struct TurtleState {
    drawing: Vec<TurtleSegment>,
    position: Coordinate,
    heading: Angle<Precision>,
    colors: TurtleColors,
    visible: Visibility,
    shape_transform: Transform,
    speed: Speed,
    segment_index: u64,
}

impl TurtleState {
    pub fn add_segment(&mut self, seg: TurtleSegment) {
        self.drawing.push(seg);
    }
}

impl TurtleState {
    pub fn segment_index(&self) -> u64 {
        self.segment_index
    }
    pub fn heading(&self) -> Angle<Precision> {
        self.heading
    }
    pub fn position(&self) -> Coordinate {
        self.position
    }
    pub fn speed(&self) -> Speed {
        self.speed
    }
    /// The duration of animations calculated from the speed.
    pub fn animation_duration(&self) -> Duration {
        Duration::from_millis(self.speed() as u64)
    }
    pub fn shape_transform(&self) -> Transform {
        self.shape_transform
    }
}
impl TurtleState {
    pub fn set_heading(&mut self, angle: Angle<Precision>) -> &mut Self {
        self.heading = angle;
        self
    }
    pub fn set_position(&mut self, position: Coordinate) -> &mut Self {
        self.position = position;
        self
    }
    pub fn set_speed(&mut self, speed: Speed) -> &mut Self {
        self.speed = max(speed, 1);
        self
    }
    pub fn set_shape_transform(&mut self, transform: Transform) -> &mut Self {
        self.shape_transform = transform;
        self
    }
}
