use bevy::reflect::{FromReflect, Reflect};

pub use self::line_segments::{TurtleDrawCircle, TurtleDrawLine};

pub mod animation;
mod line_segments;
pub(crate) mod run_step;

#[derive(Reflect, FromReflect, Default, Debug)]
pub enum TurtleGraphElement {
    TurtleLine(TurtleDrawLine),
    TurtleCircle(TurtleDrawCircle),
    #[default]
    Noop,
}
