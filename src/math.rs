//! Basic types and operations for math

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// 2D point with x and y coordinates
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    /// x coordinate
    pub x: f32,

    /// y coordinate
    pub y: f32,
}

impl Point {
    /// Create new Point on (x, y)
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Create new Point on (0.0, 0.0)
    pub const fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    /// Calculate Euclidean distance to given point
    pub fn distance_to(&self, other: &Self) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Mul for Point {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl MulAssign for Point {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
    }
}

impl Div for Point {
    type Output = Self;

    /// # Panics
    ///
    /// Panics when value of other's x or y is zero
    fn div(self, other: Self) -> Self::Output {
        if other.x == 0.0 || other.y == 0.0 {
            panic!(
                "Attempted to divide {:?} by {:?}. (division-by-zero)",
                self, other
            );
        }

        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl DivAssign for Point {
    /// # Panics
    ///
    /// Panics when value of other's x or y is zero
    fn div_assign(&mut self, other: Self) {
        if other.x == 0.0 || other.y == 0.0 {
            panic!(
                "Attempted to divide {:?} by {:?}. (division-by-zero)",
                *self, other
            );
        }

        self.x /= other.x;
        self.y /= other.y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_create_new_point() {
        let point = Point::new(1.6, 3.6);

        assert_relative_eq!(point.x, 1.6, epsilon = 1e-6);
        assert_relative_eq!(point.y, 3.6, epsilon = 1e-6);
    }

    #[test]
    fn test_create_negative_point() {
        let point = Point::new(-2.3, -51.2);

        assert_relative_eq!(point.x, -2.3, epsilon = 1e-6);
        assert_relative_eq!(point.y, -51.2, epsilon = 1e-6);
        assert!(point.x < 0.0);
        assert!(point.y < 0.0);
    }

    #[test]
    fn test_create_zero_point() {
        let point = Point::zero();

        assert_relative_eq!(point.x, 0.0, epsilon = 1e-6);
        assert_relative_eq!(point.y, 0.0, epsilon = 1e-6);
    }

    #[test]
    fn test_copy_point() {
        let point_1 = Point::new(1.6, 3.7);
        let point_2 = point_1;

        assert_relative_eq!(point_1.x, point_2.x, epsilon = 1e-6);
        assert_relative_eq!(point_1.y, point_2.y, epsilon = 1e-6);
    }

    #[test]
    fn test_clone_point() {
        let point_1 = Point::new(-1.5, 16.3);
        let point_2 = point_1.clone();

        assert_relative_eq!(point_1.x, point_2.x, epsilon = 1e-6);
        assert_relative_eq!(point_1.y, point_2.y, epsilon = 1e-6);
    }

    #[test]
    fn test_calculate_distance_between_same_point() {
        let point_1 = Point::new(1.23, 23.1);
        let point_2 = point_1.clone();

        assert_relative_eq!(point_1.distance_to(&point_2), 0.0, epsilon = 1e-6);
    }

    #[test]
    fn test_calculate_distance_between_different_point() {
        let point_1 = Point::new(-2.0, -1.5);
        let point_2 = Point::new(1.0, 2.5);

        assert_relative_eq!(point_1.distance_to(&point_2), 5.0, epsilon = 1e-6);
    }

    #[test]
    fn test_add_two_points() {
        let point_1 = Point::new(-1.0, 3.5);
        let point_2 = Point::new(-2.3, -5.2);

        let added_point = point_1 + point_2;

        assert_relative_eq!(added_point.x, point_1.x + point_2.x, epsilon = 1e-6);
        assert_relative_eq!(added_point.y, point_1.y + point_2.y, epsilon = 1e-6);
    }

    #[test]
    fn test_add_assigning_other_point() {
        let x_1 = 1.3;
        let y_1 = 6.23;
        let mut point_1 = Point::new(x_1, y_1);
        let point_2 = Point::new(23.6, 231.6);

        point_1 += point_2;

        assert_relative_eq!(point_1.x, x_1 + point_2.x, epsilon = 1e-6);
        assert_relative_eq!(point_1.y, y_1 + point_2.y, epsilon = 1e-6);
    }

    #[test]
    fn test_sub_two_points() {
        let point_1 = Point::new(-1.0, 3.5);
        let point_2 = Point::new(-2.3, -5.2);

        let subtracted_point = point_1 - point_2;

        assert_relative_eq!(subtracted_point.x, point_1.x - point_2.x, epsilon = 1e-6);
        assert_relative_eq!(subtracted_point.y, point_1.y - point_2.y, epsilon = 1e-6);
    }

    #[test]
    fn test_sub_assigning_other_point() {
        let x_1 = 1.3;
        let y_1 = 6.23;
        let mut point_1 = Point::new(x_1, y_1);
        let point_2 = Point::new(23.6, 231.6);

        point_1 -= point_2;

        assert_relative_eq!(point_1.x, x_1 - point_2.x, epsilon = 1e-6);
        assert_relative_eq!(point_1.y, y_1 - point_2.y, epsilon = 1e-6);
    }

    #[test]
    fn test_mul_two_points() {
        let point_1 = Point::new(-1.0, 3.5);
        let point_2 = Point::new(-2.3, -5.2);

        let multiplied_point = point_1 * point_2;

        assert_relative_eq!(multiplied_point.x, point_1.x * point_2.x, epsilon = 1e-6);
        assert_relative_eq!(multiplied_point.y, point_1.y * point_2.y, epsilon = 1e-6);
    }

    #[test]
    fn test_mul_assigning_other_point() {
        let x_1 = 1.3;
        let y_1 = 1.4;
        let mut point_1 = Point::new(x_1, y_1);
        let point_2 = Point::new(23.6, 231.6);

        point_1 *= point_2;

        assert_relative_eq!(point_1.x, x_1 * point_2.x, epsilon = 1e-6);
        assert_relative_eq!(point_1.y, y_1 * point_2.y, epsilon = 1e-6);
    }

    #[test]
    fn test_div_two_points() {
        let point_1 = Point::new(-1.0, 3.5);
        let point_2 = Point::new(-2.3, -5.2);

        let divided_point = point_1 / point_2;

        assert_relative_eq!(divided_point.x, point_1.x / point_2.x, epsilon = 1e-6);
        assert_relative_eq!(divided_point.y, point_1.y / point_2.y, epsilon = 1e-6);
    }

    #[test]
    fn test_div_assigning_other_point() {
        let x_1 = 1.3;
        let y_1 = 6.23;
        let mut point_1 = Point::new(x_1, y_1);
        let point_2 = Point::new(23.6, 231.6);

        point_1 /= point_2;

        assert_relative_eq!(point_1.x, x_1 / point_2.x, epsilon = 1e-6);
        assert_relative_eq!(point_1.y, y_1 / point_2.y, epsilon = 1e-6);
    }

    #[test]
    #[should_panic(expected = "division-by-zero")]
    fn test_division_by_zero_point() {
        let point_1 = Point::new(1.3, 4.3);
        let point_with_zero = Point::new(0.0, 4.3);

        let _ = point_1 / point_with_zero;
    }

    #[test]
    #[should_panic(expected = "division-by-zero")]
    fn test_div_assign_by_zero() {
        let mut point_1 = Point::new(1.0, 2.0);
        let point_with_zero = Point::new(1.0, 0.0);

        point_1 /= point_with_zero;
    }
}
