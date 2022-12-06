use bevy_inspector_egui::Inspectable;

use super::Precision;

#[derive(Inspectable, Default, Copy, Clone, Debug)]
pub struct Length(pub Precision);
