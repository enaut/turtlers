//! Length type for distance measurements

use super::Precision;

#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub struct Length(pub Precision);

impl From<i16> for Length {
    fn from(i: i16) -> Self {
        Self(Precision::from(i))
    }
}

impl From<f32> for Length {
    fn from(f: f32) -> Self {
        Self(f)
    }
}

impl From<i32> for Length {
    fn from(i: i32) -> Self {
        Self(i as Precision)
    }
}
