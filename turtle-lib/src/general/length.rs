//! Length type for distance measurements

use super::Precision;
use std::ops::Neg;

/// A spatial distance or length measurement.
///
/// Used at the public API boundary for movement distances and arc radii.
#[derive(Default, Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Length(pub Precision);

impl Length {
    /// Create a new `Length` from a raw value.
    #[must_use]
    pub const fn new(v: Precision) -> Self {
        Self(v)
    }

    /// Extract the raw `Precision` (`f32`) value.
    #[must_use]
    pub const fn value(self) -> Precision {
        self.0
    }
}

impl Neg for Length {
    type Output = Self;

    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl From<f32> for Length {
    fn from(f: f32) -> Self {
        Self(f)
    }
}

impl From<f64> for Length {
    fn from(f: f64) -> Self {
        Self(f as Precision)
    }
}

impl From<i16> for Length {
    fn from(i: i16) -> Self {
        Self(Precision::from(i))
    }
}

impl From<i32> for Length {
    fn from(i: i32) -> Self {
        Self(i as Precision)
    }
}

impl From<usize> for Length {
    fn from(u: usize) -> Self {
        Self(u as Precision)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_conversions_and_negation() {
        let l_f32: Length = 42.5_f32.into();
        assert_eq!(l_f32.value(), 42.5);

        let l_f64: Length = 100.0_f64.into();
        assert_eq!(l_f64.value(), 100.0);

        let l_i32: Length = 50_i32.into();
        assert_eq!(l_i32.value(), 50.0);

        let l_i16: Length = 25_i16.into();
        assert_eq!(l_i16.value(), 25.0);

        let l_usize: Length = 10_usize.into();
        assert_eq!(l_usize.value(), 10.0);

        let neg = -l_f32;
        assert_eq!(neg.value(), -42.5);

        assert!(Length::new(10.0) < Length::new(20.0));
    }
}


