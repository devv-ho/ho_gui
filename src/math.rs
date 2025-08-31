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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new_point() {
        let point = Point::new(1.6, 3.6);

        assert_eq!(point.x, 1.6);
        assert_eq!(point.y, 3.6);
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
}
