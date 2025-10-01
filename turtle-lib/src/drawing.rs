use bevy::reflect::Reflect;

pub use self::line_segments::{TurtleDrawCircle, TurtleDrawLine};

#[cfg(feature = "tweening")]
pub mod animation;
#[cfg(not(feature = "tweening"))]
pub(crate) mod immediate;
mod line_segments;
#[cfg(feature = "tweening")]
pub(crate) mod run_step;

#[derive(Reflect, Default, Debug)]
pub enum TurtleGraphElement {
    TurtleLine(TurtleDrawLine),
    TurtleCircle(TurtleDrawCircle),
    #[default]
    Noop,
}
