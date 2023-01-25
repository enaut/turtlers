mod turtle;
use bevy::{
    prelude::{Color, Component},
    reflect::Reflect,
};
pub use turtle::turtle;

#[derive(Clone, Component, Reflect)]
pub struct TurtleShape;

#[derive(Clone, Component, Reflect, Default, Debug)]
pub struct TurtleColors {
    color: Color,
    fill_color: Color,
}
