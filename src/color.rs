use std::{
    cmp,
    ops::{self, Mul},
};

use crate::utils;

/// Represents RGB color.
///
/// Structure is `(red, green, blue)`.
/// Addition `+`, subtraction `-` and multiplication by a scalar `* f64`,
/// multiplication by another color `*` and equality `==` supported through operator overloading.
#[derive(Clone, Copy, Debug)]
pub struct Color(f64, f64, f64);

impl ops::Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        Color(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Color(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl cmp::PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        utils::float_eq(self.0, other.0)
            && utils::float_eq(self.1, other.1)
            && utils::float_eq(self.2, other.2)
    }
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self(red, green, blue)
    }

    pub fn black() -> Self {
        Self(0.0, 0.0, 0.0)
    }

    pub fn white() -> Self {
        Self(1.0, 1.0, 1.0)
    }

    pub fn as_8bit(&self) -> (u8, u8, u8) {
        (
            (self.0.clamp(0.0, 1.0) * 255.0).round() as u8,
            (self.1.clamp(0.0, 1.0) * 255.0).round() as u8,
            (self.2.clamp(0.0, 1.0) * 255.0).round() as u8,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert!((c1 + c2) == Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn subtracting_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert!((c1 - c2) == Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn multiplying_color_by_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);
        assert!((c * 2.0) == Color::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn multiplying_two_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        assert!((c1 * c2) == Color::new(0.9, 0.2, 0.04));
    }

    #[test]
    fn color_equality_check_works() {
        assert!(Color::new(0.4, 0.08, 0.3) == Color::new(0.4, 0.08, 0.3));
        assert!(Color::new(0.4, 0.08, 0.3) != Color::new(0.4, 0.8, 0.3));
    }

    #[test]
    fn eight_bit_conversion_works() {
        let c = Color(1.0, 0.356, 0.0);
        let c = c.as_8bit();
        assert_eq!(c.0, 255);
        assert_eq!(c.1, 91);
        assert_eq!(c.2, 0);
    }
}
