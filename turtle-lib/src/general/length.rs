use bevy::reflect::Reflect;

use super::Precision;

#[derive(Reflect, Default, Copy, Clone, Debug)]
pub struct Length(pub Precision);

impl From<i16> for Length {
    fn from(i: i16) -> Self {
        Self(Precision::from(i))
    }
}

impl From<f32> for Length {
    fn from(i: f32) -> Self {
        #[allow(clippy::useless_conversion)]
        Self(Precision::from(i))
    }
}
