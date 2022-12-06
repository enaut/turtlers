mod turtle;
use bevy::prelude::{Color, Component};
use bevy_inspector_egui::Inspectable;
pub use turtle::turtle;

#[derive(Clone, Component, Inspectable)]
pub struct TurtleShape;

#[derive(Clone, Component, Inspectable, Default)]
pub struct TurtleColors {
    color: Color,
    fill_color: Color,
}
