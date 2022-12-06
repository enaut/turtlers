use bevy::prelude::Vec2;
use bevy_prototype_lyon::{
    prelude::{Path, ShapePath},
    shapes,
};
use bevy_tweening::Lens;

pub(crate) struct LineAnimationLens {
    start: Vec2,
    end: Vec2,
}

impl LineAnimationLens {
    pub(crate) fn new(start: Vec2, end: Vec2) -> Self {
        Self { start, end }
    }
}

impl Lens<Path> for LineAnimationLens {
    fn lerp(&mut self, target: &mut Path, ratio: f32) {
        let line = shapes::Line(self.start, self.start + ((self.end - self.start) * ratio));
        *target = ShapePath::build_as(&line);
    }
}
