//! Basic types and operations for math

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

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_create_new_point() {
        let point = Point::new(1.6, 3.6);

        assert_eq!(point.x, 1.6);
        assert_eq!(point.y, 3.6);
    }

    #[test]
    fn test_create_negative_point() {
        let point = Point::new(-2.3, -51.2);

        assert_eq!(point.x, -2.3);
        assert_eq!(point.y, -51.2);
        assert!(point.x < 0.0);
        assert!(point.y < 0.0);
    }

    #[test]
    fn test_create_zero_point() {
        let point = Point::zero();

        assert_eq!(point.x, 0.0);
        assert_eq!(point.y, 0.0);
    }

    #[test]
    fn test_copy_point() {
        let point_1 = Point::new(1.6, 3.7);
        let point_2 = point_1;

        assert_eq!(point_1.x, point_2.x);
        assert_eq!(point_1.y, point_2.y);
    }

    #[test]
    fn test_clone_point() {
        let point_1 = Point::new(-1.5, 16.3);
        let point_2 = point_1.clone();

        assert_eq!(point_1.x, point_2.x);
        assert_eq!(point_1.y, point_2.y);
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
}
