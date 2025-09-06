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
            panic!("Attempted to divide {self:?} by {other:?}. (division-by-zero)");
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

/// Size for rectangle
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    /// width of a rectangle
    pub width: f32,

    /// height of a rectangle
    pub height: f32,
}

impl Size {
    /// Create new Size with specified width and height
    pub const fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    /// Create new Size with zero width and height
    pub const fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    /// Check if width and height is not negative
    pub fn is_valid(&self) -> bool {
        self.width >= 0.0 && self.height >= 0.0
    }

    /// Check if width and height is positive
    pub fn is_positive(&self) -> bool {
        self.width > 0.0 && self.height > 0.0
    }

    /// Calculate area of a Size
    pub fn area(&self) -> f32 {
        if !self.is_valid() {
            panic!("Attempted to get area of an invalid size. (invalid-argument)");
        }

        self.width * self.height
    }
}

/// Rectangle with position and size
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    /// Position of top-left point of a rectangle
    pub pos: Point,

    /// Size of a rectangle
    pub size: Size,
}

impl Rect {
    /// Create a rectangle with specified position and size
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            pos: Point::new(x, y),
            size: Size::new(width, height),
        }
    }

    /// Create a rectangle positioned on zero point and with zero size
    pub const fn zero() -> Self {
        Self {
            pos: Point::zero(),
            size: Size::zero(),
        }
    }

    /// Get x coordinate of left edge
    pub fn left(&self) -> f32 {
        self.pos.x
    }

    /// Get x coordinate of right edge
    pub fn right(&self) -> f32 {
        self.pos.x + self.size.width
    }

    /// Get y coordinate of top edge
    pub fn top(&self) -> f32 {
        self.pos.y
    }

    /// Get y coordinate of bottom edge
    pub fn bottom(&self) -> f32 {
        self.pos.y + self.size.height
    }

    /// Check if rectangle contains a point.
    ///
    /// NOTE: returns true if point is on the edge
    pub fn contains_point(&self, point: Point) -> bool {
        point.x >= self.left()
            && point.x <= self.right()
            && point.y <= self.bottom()
            && point.y >= self.top()
    }

    /// Check if rectangle intersects other rectangle
    ///
    /// NOTE: return true if rectangle are touching each other, and smaller rect returns true if it
    /// is located inside of bigger rect
    pub fn intersects(&self, other: Self) -> bool {
        self.left() <= other.right()
            && self.right() >= other.left()
            && self.top() <= other.bottom()
            && self.bottom() >= other.top()
    }

    /// Get area of a rectangle
    pub fn area(&self) -> f32 {
        self.size.area()
    }
}

pub type Vec2 = Point;

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

    #[test]
    fn test_new_size() {
        let width = 1.3;
        let height = 3.5;

        let size = Size::new(width, height);

        assert_relative_eq!(width, size.width, epsilon = 1e-6);
        assert_relative_eq!(height, size.height, epsilon = 1e-6);
    }

    #[test]
    fn test_zero_size() {
        let zero_size = Size::zero();

        assert_relative_eq!(zero_size.width, 0.0, epsilon = 1e-6);
        assert_relative_eq!(zero_size.height, 0.0, epsilon = 1e-6);
    }

    #[test]
    fn test_valid_size() {
        let width = 0.0;
        let height = 2.5;

        let valid_size = Size::new(width, height);

        assert!(valid_size.is_valid());
    }

    #[test]
    fn test_invalid_size() {
        let width = 0.0;
        let height = -1.2;

        let invalid_size = Size::new(width, height);

        assert!(!invalid_size.is_valid());
    }

    #[test]
    fn test_zero_size_validity() {
        let zero_size = Size::zero();

        assert!(zero_size.is_valid());
    }

    #[test]
    fn test_positive_size_is_positive() {
        let positive_size = Size::new(1.65, 34.1);

        assert!(positive_size.is_positive());
    }

    #[test]
    fn test_zero_size_is_positive() {
        let zero_size = Size::zero();

        assert!(!zero_size.is_positive());
    }

    #[test]
    fn test_get_area_of_valid_size() {
        let size = Size::new(23.0, 3.0);

        assert_eq!(size.area(), size.width * size.height);
    }

    #[test]
    #[should_panic(expected = "Attempted to get area of an invalid size. (invalid-argument)")]
    fn test_get_area_of_invalid_size() {
        let invalid_size = Size::new(-1.0, 4.0);

        let _ = invalid_size.area();
    }

    #[test]
    fn test_rect_creation() {
        let pos = Point::new(1.5, 2.3);
        let size = Size::new(10.3, 35.1);

        let rect = Rect::new(pos.x, pos.y, size.width, size.height);

        assert_relative_eq!(rect.pos.x, pos.x, epsilon = 1e-6);
        assert_relative_eq!(rect.pos.y, pos.y, epsilon = 1e-6);
        assert_relative_eq!(rect.size.width, size.width, epsilon = 1e-6);
        assert_relative_eq!(rect.size.height, size.height, epsilon = 1e-6);
    }

    #[test]
    fn test_zero_rect_creation() {
        let zero_rect = Rect::zero();

        assert_relative_eq!(zero_rect.pos.x, 0.0, epsilon = 1e-6);
        assert_relative_eq!(zero_rect.pos.y, 0.0, epsilon = 1e-6);
        assert_relative_eq!(zero_rect.size.width, 0.0, epsilon = 1e-6);
        assert_relative_eq!(zero_rect.size.height, 0.0, epsilon = 1e-6);
    }

    #[test]
    fn test_left_of_rect() {
        let pos = Point::new(-1.23, 23.41);
        let size = Size::new(10.23, 21.4);

        let rect = Rect::new(pos.x, pos.y, size.width, size.height);

        assert_relative_eq!(rect.left(), pos.x);
    }

    #[test]
    fn test_right_of_rect() {
        let pos = Point::new(-1.23, 23.41);
        let size = Size::new(10.23, 21.4);

        let rect = Rect::new(pos.x, pos.y, size.width, size.height);

        assert_relative_eq!(rect.right(), pos.x + size.width);
    }

    #[test]
    fn test_top_of_rect() {
        let pos = Point::new(-1.23, 23.41);
        let size = Size::new(10.23, 21.4);

        let rect = Rect::new(pos.x, pos.y, size.width, size.height);

        assert_relative_eq!(rect.top(), pos.y);
    }

    #[test]
    fn test_bottom_of_rect() {
        let pos = Point::new(-1.23, 23.41);
        let size = Size::new(10.23, 21.4);

        let rect = Rect::new(pos.x, pos.y, size.width, size.height);

        assert_relative_eq!(rect.bottom(), pos.y + size.height);
    }

    #[test]
    fn test_rect_not_contains_point_outside() {
        let rect = Rect::new(0.0, 0.0, 10.3, 175.3);
        let point_out_of_rect = Point::new(-1.0, -2.3);

        assert_eq!(rect.contains_point(point_out_of_rect), false);
    }

    #[test]
    fn test_rect_contains_point_on_edge() {
        let rect = Rect::new(0.0, 0.0, 10.3, 175.3);

        let point_on_left_edge = Point::new(0.0, 50.3);
        let point_on_right_edge = Point::new(10.3, 23.5);
        let point_on_top_edge = Point::new(5.3, 0.0);
        let point_on_bottom_edge = Point::new(4.1, 175.3);

        assert_eq!(rect.contains_point(point_on_left_edge), true);
        assert_eq!(rect.contains_point(point_on_right_edge), true);
        assert_eq!(rect.contains_point(point_on_top_edge), true);
        assert_eq!(rect.contains_point(point_on_bottom_edge), true);
    }

    #[test]
    fn test_rect_contains_point_inside() {
        let rect = Rect::new(0.0, 0.0, 10.3, 175.3);

        let point_inside_rect = Point::new(4.6, 36.3);

        assert_eq!(rect.contains_point(point_inside_rect), true);
    }

    #[test]
    fn test_rect_not_intersects() {
        let rect_1 = Rect::new(0.0, 0.0, 10.0, 10.0);
        let rect_2 = Rect::new(20.0, 20.0, 10.0, 10.0);

        assert_eq!(rect_1.intersects(rect_2), false);
    }

    #[test]
    fn test_rect_intersects_touched_rect() {
        let rect = Rect::new(0.0, 0.0, 10.0, 10.0);
        let rect_touch_left = Rect::new(-5.0, 2.0, 5.0, 5.0);
        let rect_touch_right = Rect::new(10.0, 2.0, 5.0, 5.0);
        let rect_touch_top = Rect::new(2.0, -5.0, 5.0, 5.0);
        let rect_touch_bottom = Rect::new(2.0, 10.0, 5.0, 5.0);

        assert_eq!(rect.intersects(rect_touch_left), true);
        assert_eq!(rect.intersects(rect_touch_right), true);
        assert_eq!(rect.intersects(rect_touch_top), true);
        assert_eq!(rect.intersects(rect_touch_bottom), true);
    }

    #[test]
    fn test_rect_intersects_crossed_rect() {
        let rect = Rect::new(0.0, 0.0, 10.0, 10.0);
        let rect_crossed_left = Rect::new(-5.0, 2.0, 8.0, 5.0);
        let rect_crossed_right = Rect::new(8.0, 2.0, 5.0, 5.0);
        let rect_crossed_top = Rect::new(2.0, -5.0, 5.0, 8.0);
        let rect_crossed_bottom = Rect::new(2.0, 8.0, 5.0, 5.0);

        assert_eq!(rect.intersects(rect_crossed_left), true);
        assert_eq!(rect.intersects(rect_crossed_right), true);
        assert_eq!(rect.intersects(rect_crossed_top), true);
        assert_eq!(rect.intersects(rect_crossed_bottom), true);
    }

    #[test]
    fn test_bigger_rect_intersects_smaller_rect() {
        let smaller_rect = Rect::new(2.0, 2.0, 2.0, 2.0);
        let bigger_rect = Rect::new(0.0, 0.0, 10.0, 10.0);

        assert_eq!(bigger_rect.intersects(smaller_rect), true);
    }

    #[test]
    fn test_smaller_rect_intersects_bigger_rect() {
        let smaller_rect = Rect::new(2.0, 2.0, 2.0, 2.0);
        let bigger_rect = Rect::new(0.0, 0.0, 10.0, 10.0);

        assert_eq!(smaller_rect.intersects(bigger_rect), true);
    }

    #[test]
    fn test_area_of_rect() {
        let rect = Rect::new(0.0, 0.0, 10.0, 23.0);

        assert_relative_eq!(
            rect.area(),
            rect.size.width * rect.size.height,
            epsilon = 1e-6
        );
    }
}
