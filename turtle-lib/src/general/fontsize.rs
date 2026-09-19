//! `FontSize` type for text rendering

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FontSize(pub u16);

impl FontSize {
    /// Create a new `FontSize` from a u16 value
    #[must_use]
    pub const fn new(size: u16) -> Self {
        Self(size)
    }

    /// Get the inner u16 value
    #[must_use]
    pub const fn value(self) -> u16 {
        self.0
    }
}

impl From<u16> for FontSize {
    fn from(size: u16) -> Self {
        Self(size)
    }
}

impl From<f32> for FontSize {
    fn from(f: f32) -> Self {
        Self(f.max(1.0) as u16)
    }
}

impl From<i32> for FontSize {
    fn from(i: i32) -> Self {
        Self(i.max(1) as u16)
    }
}

impl From<i16> for FontSize {
    fn from(i: i16) -> Self {
        Self(i.max(1) as u16)
    }
}

impl From<f64> for FontSize {
    fn from(f: f64) -> Self {
        Self(f.max(1.0) as u16)
    }
}

impl From<usize> for FontSize {
    fn from(size: usize) -> Self {
        Self((size as u16).max(1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn font_size_conversions() {
        assert_eq!(FontSize::from(16_u16).value(), 16);
        assert_eq!(FontSize::from(24_i32).value(), 24);
        assert_eq!(FontSize::from(18_i16).value(), 18);
        assert_eq!(FontSize::from(32_usize).value(), 32);
        assert_eq!(FontSize::from(20.5_f32).value(), 20);
        assert_eq!(FontSize::from(28.0_f64).value(), 28);
    }
}
