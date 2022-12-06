use bevy_inspector_egui::Inspectable;

pub use self::line_segments::{TurtleDrawCircle, TurtleDrawLine};

pub mod animation;
mod line_segments;
pub(crate) mod run_step;

#[derive(Inspectable, Default)]
pub enum TurtleGraphElement {
    TurtleLine(TurtleDrawLine),
    TurtleCircle(TurtleDrawCircle),
    #[default]
    Noop,
}
