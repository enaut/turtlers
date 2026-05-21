//! Angle unit newtypes: `Degrees` and `Radians`.
//!
//! ## Design
//!
//! Two separate types instead of a single enum so that function signatures are
//! self-documenting and the compiler rejects wrong-unit arguments.
//!
//! - **`Degrees`** — public API boundary. Builder methods and `TurtleCommand`
//!   fields that originate from user input store this type. Convert with
//!   `as_radians()` before entering the rendering pipeline.
//!
//! - **`Radians`** — internal pipeline. All geometry functions and
//!   `TurtleParams` arithmetic work in radians. Extract the raw `f32` with
//!   `value()` only where stdlib trig functions (`sin`, `cos`, …) require it.
//!
//! There is intentionally **no** conversion from `Radians` back to `f32` that
//! strips the unit tag silently — use `.value()` explicitly and at the last
//! possible moment.

use super::Precision;
use std::ops::Neg;

/// An angle measured in degrees.
///
/// Used at the public API boundary. Convert to [`Radians`] with `as_radians()`
/// before passing into internal rendering functions.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct Degrees(pub Precision);

impl Degrees {
    /// Construct from a raw degrees value.
    #[must_use]
    pub fn new(v: Precision) -> Self {
        Self(v)
    }

    /// Convert to [`Radians`] for use in the rendering pipeline.
    ///
    /// This is the **only** correct way to enter the internal math layer.
    #[must_use]
    pub fn as_radians(self) -> Radians {
        Radians(self.0.to_radians())
    }

    /// The raw degrees value.
    ///
    /// Use only for degree-to-degree arithmetic (e.g. negating a turn angle
    /// before storing it as a command). Do not pass this to trig functions.
    #[must_use]
    pub fn value(self) -> Precision {
        self.0
    }
}

impl Neg for Degrees {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl From<f32> for Degrees {
    fn from(v: f32) -> Self {
        Self(v)
    }
}

impl From<i32> for Degrees {
    fn from(v: i32) -> Self {
        Self(v as Precision)
    }
}

impl From<i16> for Degrees {
    fn from(v: i16) -> Self {
        Self(Precision::from(v))
    }
}

// ─────────────────────────────────────────────────────────────────────────────

/// An angle measured in radians.
///
/// Used in all internal function signatures and geometry math.  Extract the
/// raw `f32` with [`value()`](Radians::value) only when calling stdlib trig
/// functions (`sin`, `cos`, etc.).
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct Radians(pub Precision);

impl Radians {
    /// Construct from a raw radians value.
    #[must_use]
    pub fn new(v: Precision) -> Self {
        Self(v)
    }

    /// Convert to [`Degrees`] for display or user-facing output.
    #[must_use]
    pub fn as_degrees(self) -> Degrees {
        Degrees(self.0.to_degrees())
    }

    /// The raw radians value.
    ///
    /// Use only when calling stdlib trig functions or other `f32`-based
    /// math APIs. Keep `Radians` as the type at all internal function
    /// boundaries.
    #[must_use]
    pub fn value(self) -> Precision {
        self.0
    }
}

impl Neg for Radians {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl From<f32> for Radians {
    fn from(v: f32) -> Self {
        Self(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn degrees_to_radians_roundtrip() {
        let deg = Degrees::new(180.0);
        let rad = deg.as_radians();
        assert!(
            (rad.value() - PI).abs() < 1e-6,
            "expected π, got {}",
            rad.value()
        );
        let back = rad.as_degrees();
        assert!(
            (back.value() - 180.0).abs() < 1e-4,
            "expected 180°, got {}",
            back.value()
        );
    }

    #[test]
    fn negation() {
        assert_eq!(-Degrees::new(90.0), Degrees::new(-90.0));
        assert_eq!(-Radians::new(1.0), Radians::new(-1.0));
    }

    #[test]
    fn from_integer() {
        let d: Degrees = 90_i32.into();
        assert_eq!(d, Degrees::new(90.0));
        let d2: Degrees = 45_i16.into();
        assert_eq!(d2, Degrees::new(45.0));
    }
}
