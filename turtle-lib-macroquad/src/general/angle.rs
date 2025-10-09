//! Angle type with degrees and radians support

use super::Precision;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AngleUnit {
    Degrees(Precision),
    Radians(Precision),
}

impl Default for AngleUnit {
    fn default() -> Self {
        Self::Degrees(0.0)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Angle {
    value: AngleUnit,
}

impl Default for Angle {
    fn default() -> Self {
        Self {
            value: AngleUnit::Degrees(0.0),
        }
    }
}

impl From<i16> for Angle {
    fn from(i: i16) -> Self {
        Self {
            value: AngleUnit::Degrees(i as Precision),
        }
    }
}

impl From<f32> for Angle {
    fn from(f: f32) -> Self {
        Self {
            value: AngleUnit::Degrees(f),
        }
    }
}

impl Rem<Precision> for Angle {
    type Output = Self;

    fn rem(self, rhs: Precision) -> Self::Output {
        match self.value {
            AngleUnit::Degrees(v) => Self::degrees(v % rhs),
            AngleUnit::Radians(v) => Self::radians(v % rhs),
        }
    }
}

impl Mul<Precision> for Angle {
    type Output = Self;

    fn mul(self, rhs: Precision) -> Self::Output {
        match self.value {
            AngleUnit::Degrees(v) => Self::degrees(v * rhs),
            AngleUnit::Radians(v) => Self::radians(v * rhs),
        }
    }
}

impl Div<Precision> for Angle {
    type Output = Self;

    fn div(self, rhs: Precision) -> Self::Output {
        match self.value {
            AngleUnit::Degrees(v) => Self::degrees(v / rhs),
            AngleUnit::Radians(v) => Self::radians(v / rhs),
        }
    }
}

impl Neg for Angle {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self.value {
            AngleUnit::Degrees(v) => Self::degrees(-v),
            AngleUnit::Radians(v) => Self::radians(-v),
        }
    }
}

impl Neg for &Angle {
    type Output = Angle;

    fn neg(self) -> Self::Output {
        match self.value {
            AngleUnit::Degrees(v) => Angle::degrees(-v),
            AngleUnit::Radians(v) => Angle::radians(-v),
        }
    }
}

impl Add for Angle {
    type Output = Angle;

    fn add(self, rhs: Self) -> Self::Output {
        match (self.value, rhs.value) {
            (AngleUnit::Degrees(v), AngleUnit::Degrees(o)) => Self::degrees(v + o),
            (AngleUnit::Degrees(v), AngleUnit::Radians(o)) => Self::radians(v.to_radians() + o),
            (AngleUnit::Radians(v), AngleUnit::Degrees(o)) => Self::radians(v + o.to_radians()),
            (AngleUnit::Radians(v), AngleUnit::Radians(o)) => Self::radians(v + o),
        }
    }
}

impl Sub for Angle {
    type Output = Angle;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self.value, rhs.value) {
            (AngleUnit::Degrees(v), AngleUnit::Degrees(o)) => Self::degrees(v - o),
            (AngleUnit::Degrees(v), AngleUnit::Radians(o)) => Self::radians(v.to_radians() - o),
            (AngleUnit::Radians(v), AngleUnit::Degrees(o)) => Self::radians(v - o.to_radians()),
            (AngleUnit::Radians(v), AngleUnit::Radians(o)) => Self::radians(v - o),
        }
    }
}

impl Angle {
    pub fn degrees(value: Precision) -> Self {
        Self {
            value: AngleUnit::Degrees(value),
        }
    }

    pub fn radians(value: Precision) -> Self {
        Self {
            value: AngleUnit::Radians(value),
        }
    }

    pub fn value(&self) -> Precision {
        match self.value {
            AngleUnit::Degrees(v) => v,
            AngleUnit::Radians(v) => v,
        }
    }

    pub fn to_radians(self) -> Self {
        match self.value {
            AngleUnit::Degrees(v) => Self::radians(v.to_radians()),
            AngleUnit::Radians(_) => self,
        }
    }

    pub fn to_degrees(self) -> Self {
        match self.value {
            AngleUnit::Degrees(_) => self,
            AngleUnit::Radians(v) => Self::degrees(v.to_degrees()),
        }
    }

    pub fn limit_smaller_than_full_circle(self) -> Self {
        use std::f32::consts::PI;
        match self.value {
            AngleUnit::Degrees(v) => Self::degrees(v % 360.0),
            AngleUnit::Radians(v) => Self::radians(v % (2.0 * PI)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_to_radians() {
        let radi = Angle::radians(30f32.to_radians());
        let degr = Angle::degrees(30f32);
        let converted = degr.to_radians();
        assert!((radi.value() - converted.value()).abs() < 0.0001);
    }

    #[test]
    fn sum_degrees() {
        let fst = Angle::degrees(30f32);
        let snd = Angle::degrees(30f32);
        let sum = fst + snd;
        assert!((sum.value() - 60f32).abs() < 0.0001);
        assert!((sum.to_radians().value() - 60f32.to_radians()).abs() < 0.0001);
    }

    #[test]
    fn sum_mixed() {
        let fst = Angle::degrees(30f32);
        let snd = Angle::radians(30f32.to_radians());
        let sum = fst + snd;
        assert!((sum.to_degrees().value() - 60f32).abs() < 0.0001);
        assert!((sum.to_radians().value() - 60f32.to_radians()).abs() < 0.0001);
    }
}
