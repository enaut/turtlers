use bevy_inspector_egui::Inspectable;
use std::{
    f32::consts::PI,
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};

use super::Precision;

#[derive(Inspectable, Copy, Clone, Debug, PartialEq, Eq)]
pub enum AngleUnit<T: Default> {
    Degrees(T),
    Radians(T),
}

impl<T: Default> Default for AngleUnit<T> {
    fn default() -> Self {
        Self::Degrees(Default::default())
    }
}

#[derive(Inspectable, Copy, Default, Clone, Debug, PartialEq, Eq)]
pub struct Angle<T: Default> {
    value: AngleUnit<T>,
}

impl<T: From<i16> + Default> From<i16> for Angle<T> {
    fn from(i: i16) -> Self {
        Self {
            value: AngleUnit::Degrees(T::from(i)),
        }
    }
}

impl<T: Default + Clone + Rem<T, Output = T>> Rem<T> for Angle<T> {
    type Output = Self;

    fn rem(self, rhs: T) -> Self::Output {
        match self.value {
            AngleUnit::Degrees(v) => Self::Output::degrees(v % rhs),
            AngleUnit::Radians(v) => Self::Output::radians(v % rhs),
        }
    }
}

impl<T: Default + Clone + Mul<T, Output = T>> Mul<T> for Angle<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        match self.value {
            AngleUnit::Degrees(v) => Self::Output::degrees(v * rhs),
            AngleUnit::Radians(v) => Self::Output::radians(v * rhs),
        }
    }
}

impl Angle<Precision> {
    pub fn limit_smaller_than_full_circle(self) -> Self {
        match self.value {
            AngleUnit::Degrees(v) => Self {
                value: AngleUnit::Degrees(v % 360.),
            },
            AngleUnit::Radians(v) => Self {
                value: AngleUnit::Radians(v % (2. * PI)),
            },
        }
    }
}
impl<T: Default + Clone + Div<T, Output = T>> Div<T> for Angle<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        match self.value {
            AngleUnit::Degrees(v) => Self::Output::degrees(v / rhs),
            AngleUnit::Radians(v) => Self::Output::radians(v / rhs),
        }
    }
}

impl<T: Default + Clone + std::ops::Neg<Output = T>> Neg for Angle<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self.value {
            AngleUnit::Degrees(v) => Self::Output::degrees(-v),
            AngleUnit::Radians(v) => Self::Output::radians(-v),
        }
    }
}

impl<T: Default + Clone + std::ops::Neg<Output = T>> Neg for &Angle<T> {
    type Output = Angle<T>;

    fn neg(self) -> Self::Output {
        match self.value.clone() {
            AngleUnit::Degrees(v) => Self::Output::degrees(-v),
            AngleUnit::Radians(v) => Self::Output::radians(-v),
        }
    }
}

impl<T: Default + Clone> Angle<T> {
    pub fn degrees(value: T) -> Angle<T> {
        Self {
            value: AngleUnit::Degrees(value),
        }
    }
    pub fn radians(value: T) -> Angle<T> {
        Self {
            value: AngleUnit::Radians(value),
        }
    }
    pub fn value(&self) -> T {
        match self.value.clone() {
            AngleUnit::Degrees(v) => v,
            AngleUnit::Radians(v) => v,
        }
    }
}

impl<T: Default + num_traits::float::Float> Angle<T> {
    pub fn to_radians(self) -> Self {
        match self.value {
            AngleUnit::Degrees(v) => Self {
                value: AngleUnit::Radians(v.to_radians()),
            },
            AngleUnit::Radians(_) => self,
        }
    }
    pub fn to_degrees(self) -> Self {
        match self.value {
            AngleUnit::Degrees(_) => self,
            AngleUnit::Radians(v) => Self {
                value: AngleUnit::Degrees(v.to_degrees()),
            },
        }
    }
}

impl<T: Add<Output = T> + Default + num_traits::float::Float> Add for Angle<T> {
    type Output = Angle<T>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self.value, rhs.value) {
            (AngleUnit::Degrees(v), AngleUnit::Degrees(o)) => Self::Output {
                value: AngleUnit::Degrees(v + o),
            },
            (AngleUnit::Degrees(v), AngleUnit::Radians(o)) => Self::Output {
                value: AngleUnit::Radians(v.to_radians() + o),
            },
            (AngleUnit::Radians(v), AngleUnit::Degrees(o)) => Self::Output {
                value: AngleUnit::Radians(v + o.to_radians()),
            },
            (AngleUnit::Radians(v), AngleUnit::Radians(o)) => Self::Output {
                value: AngleUnit::Radians(v + o),
            },
        }
    }
}

impl<T: Sub<Output = T> + Default + num_traits::float::Float> Sub for Angle<T> {
    type Output = Angle<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self.value, rhs.value) {
            (AngleUnit::Degrees(v), AngleUnit::Degrees(o)) => Self::Output {
                value: AngleUnit::Degrees(v - o),
            },
            (AngleUnit::Degrees(v), AngleUnit::Radians(o)) => Self::Output {
                value: AngleUnit::Radians(v.to_radians() - o),
            },
            (AngleUnit::Radians(v), AngleUnit::Degrees(o)) => Self::Output {
                value: AngleUnit::Radians(v - o.to_radians()),
            },
            (AngleUnit::Radians(v), AngleUnit::Radians(o)) => Self::Output {
                value: AngleUnit::Radians(v - o),
            },
        }
    }
}

#[test]
fn convert_to_radians() {
    let radi = Angle::radians(30f32.to_radians());
    let degr = Angle::degrees(30f32);
    let converted = degr.to_radians();
    assert_eq!(radi, converted)
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
