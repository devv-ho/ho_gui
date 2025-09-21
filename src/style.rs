//! Padding for rectangle, square components

use crate::color::Color;

/// Padding inside of rectangle, square components
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Padding {
    /// Left padding
    pub left: f32,

    /// Right padding
    pub right: f32,

    /// Top padding
    pub top: f32,

    /// Bottom padding
    pub bottom: f32,
}

impl Padding {
    /// Create Padding object
    ///
    /// # Notes
    ///
    /// - Values lower than 0.0, or `f32::NAN` will be set to 0.0
    /// - `f32::INFINITY` is preserved as valid padding.
    ///
    /// # Arguments
    ///
    /// * `left` - Left pad
    /// * `right` - Right pad
    /// * `top` - Top pad
    /// * `bottom` - Bottom pad
    ///
    /// # Returns
    ///
    /// Returns `Padding` with specified `(l, r, t, b)` padding properties
    ///
    /// # Examples
    /// ```
    /// use ho_gui::style::Padding;
    ///
    /// let pad_property = Padding::new(1.0, f32::NAN, 1.0, -1.0);
    ///
    /// assert_eq!(pad_property.left, 1.0);
    /// assert_eq!(pad_property.right, 0.0); // NaN set to 0.0
    /// assert_eq!(pad_property.top, 1.0);
    /// assert_eq!(pad_property.bottom, 0.0); // values under 0.0 set to 0.0
    /// ```
    pub const fn new(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self {
            left: Self::to_valid(left),
            right: Self::to_valid(right),
            top: Self::to_valid(top),
            bottom: Self::to_valid(bottom),
        }
    }

    /// Create a Padding object with all sides (left, right, top, bottom) set to the same value
    ///
    /// # Notes
    ///
    /// - Values lower than 0.0, or `f32::NAN` will be set to 0.0
    /// - `f32::INFINITY` is preserved as valid padding.
    ///
    /// # Arguments
    ///
    /// * `value` - Padding value, which will be set to all sides
    ///
    /// # Returns
    ///
    /// Returns `Padding { left: value, right: value, top: value, bottom: value }`
    ///
    /// # Examples
    /// ```
    /// use ho_gui::style::Padding;
    ///
    /// let pad_property = Padding::all(1.0);
    /// let nan_pad = Padding::all(f32::NAN);
    /// let negative_pad = Padding::all(-1.0);
    ///
    /// // every side set to same value (1.0)
    /// assert_eq!(pad_property.left, 1.0);
    /// assert_eq!(pad_property.right, 1.0);
    /// assert_eq!(pad_property.top, 1.0);
    /// assert_eq!(pad_property.bottom, 1.0);
    ///
    /// // f32::NAN set to 0.0
    /// assert_eq!(nan_pad.left, 0.0);
    /// assert_eq!(nan_pad.right, 0.0);
    /// assert_eq!(nan_pad.top, 0.0);
    /// assert_eq!(nan_pad.bottom, 0.0);
    ///
    /// // values under 0.0 set to 0.0
    /// assert_eq!(negative_pad.left, 0.0);
    /// assert_eq!(negative_pad.right, 0.0);
    /// assert_eq!(negative_pad.top, 0.0);
    /// assert_eq!(negative_pad.bottom, 0.0);
    /// ```
    pub const fn all(value: f32) -> Self {
        Self::new(value, value, value, value)
    }

    /// Create a Padding object with only horizontal paddings (left, right)
    ///
    /// # Notes
    ///
    /// - Vertical paddings (top, bottom) will be set to 0.0
    /// - `f32::NAN` or values under 0.0 will be set to 0.0
    /// - `f32::INFINITY` is preserved as valid padding.
    ///
    /// # Arguments
    ///
    /// * `value` - Padding value, which will be set to horizontal sides (left, right)
    ///
    /// # Returns
    ///
    /// Returns `Padding { left: value, right: value, top: 0.0, bottom: 0.0 }`
    ///
    /// # Examples
    /// ```
    /// use ho_gui::style::Padding;
    ///
    /// let pad_property = Padding::horizontal(1.0);
    /// let nan_pad = Padding::horizontal(f32::NAN);
    /// let negative_pad = Padding::horizontal(-1.0);
    ///
    /// // horizontal side set to same value (1.0), and vertical to 0.0
    /// assert_eq!(pad_property.left, 1.0);
    /// assert_eq!(pad_property.right, 1.0);
    /// assert_eq!(pad_property.top, 0.0);
    /// assert_eq!(pad_property.bottom, 0.0);
    ///
    /// // f32::NAN set to 0.0
    /// assert_eq!(nan_pad.left, 0.0);
    /// assert_eq!(nan_pad.right, 0.0);
    /// assert_eq!(nan_pad.top, 0.0);
    /// assert_eq!(nan_pad.bottom, 0.0);
    ///
    /// // values under 0.0 set to 0.0
    /// assert_eq!(negative_pad.left, 0.0);
    /// assert_eq!(negative_pad.right, 0.0);
    /// assert_eq!(negative_pad.top, 0.0);
    /// assert_eq!(negative_pad.bottom, 0.0);
    /// ```
    pub const fn horizontal(value: f32) -> Self {
        Self::new(value, value, 0.0, 0.0)
    }

    /// Create a Padding object with only vertical paddings (top, bottom)
    ///
    /// # Notes
    ///
    /// - Horizontal paddings (left, right) will be set to 0.0
    /// - `f32::NAN` or values under 0.0 will be set to 0.0
    /// - `f32::INFINITY` is preserved as valid padding.
    ///
    /// # Arguments
    ///
    /// * `value` - Padding value, which will be set to vertical sides (top, bottom)
    ///
    /// # Returns
    ///
    /// Returns `Padding { left: 0.0, right: 0.0, top: value, bottom: value }`
    ///
    /// # Examples
    /// ```
    /// use ho_gui::style::Padding;
    ///
    /// let pad_property = Padding::vertical(1.0);
    /// let nan_pad = Padding::vertical(f32::NAN);
    /// let negative_pad = Padding::vertical(-1.0);
    ///
    /// // vertical side set to same value (1.0), and horizontal to 0.0
    /// assert_eq!(pad_property.left, 0.0);
    /// assert_eq!(pad_property.right, 0.0);
    /// assert_eq!(pad_property.top, 1.0);
    /// assert_eq!(pad_property.bottom, 1.0);
    ///
    /// // f32::NAN set to 0.0
    /// assert_eq!(nan_pad.left, 0.0);
    /// assert_eq!(nan_pad.right, 0.0);
    /// assert_eq!(nan_pad.top, 0.0);
    /// assert_eq!(nan_pad.bottom, 0.0);
    ///
    /// // values under 0.0 set to 0.0
    /// assert_eq!(negative_pad.left, 0.0);
    /// assert_eq!(negative_pad.right, 0.0);
    /// assert_eq!(negative_pad.top, 0.0);
    /// assert_eq!(negative_pad.bottom, 0.0);
    /// ```
    pub const fn vertical(value: f32) -> Self {
        Self::new(0.0, 0.0, value, value)
    }

    /// Create a Padding object with specified horizontal (left, right), and vertical paddings (top, bottom)
    ///
    /// # Notes
    ///
    /// - `f32::NAN` or values under 0.0 will be set to 0.0
    /// - `f32::INFINITY` is preserved as valid padding.
    ///
    /// # Arguments
    ///
    /// * `horizontal` - Horizontal (left, right) padding value
    /// * `vertical` - Vertical (top, bottom) padding value
    ///
    /// # Returns
    ///
    /// Returns `Padding { left: horizontal, right: horizontal, top: vertical, bottom: vertical }`
    ///
    /// # Examples
    /// ```
    /// use ho_gui::style::Padding;
    ///
    /// let pad_property = Padding::symmetric(1.0, 2.0);
    /// let nan_pad = Padding::symmetric(f32::NAN, f32::NAN);
    /// let negative_pad = Padding::symmetric(-1.0, -1.0);
    ///
    /// // horizontal sides set to a horizontal value (1.0), and vertical sides set to vertical value (0.0)
    /// assert_eq!(pad_property.left, 1.0);
    /// assert_eq!(pad_property.right, 1.0);
    /// assert_eq!(pad_property.top, 2.0);
    /// assert_eq!(pad_property.bottom, 2.0);
    ///
    /// // f32::NAN set to 0.0
    /// assert_eq!(nan_pad.left, 0.0);
    /// assert_eq!(nan_pad.right, 0.0);
    /// assert_eq!(nan_pad.top, 0.0);
    /// assert_eq!(nan_pad.bottom, 0.0);
    ///
    /// // values under 0.0 set to 0.0
    /// assert_eq!(negative_pad.left, 0.0);
    /// assert_eq!(negative_pad.right, 0.0);
    /// assert_eq!(negative_pad.top, 0.0);
    /// assert_eq!(negative_pad.bottom, 0.0);
    /// ```
    pub const fn symmetric(horizontal: f32, vertical: f32) -> Self {
        Self::new(horizontal, horizontal, vertical, vertical)
    }

    /// Create a Padding object with all sides (left, right, top, bottom) set to 0.0
    ///
    /// # Returns
    ///
    /// Returns `Padding { left: 0.0, right: 0.0, top: 0.0, bottom: 0.0 }`
    ///
    /// # Examples
    /// ```
    /// use ho_gui::style::Padding;
    ///
    /// let zero_pad = Padding::zero();
    ///
    /// // all sides set to 0.0
    /// assert_eq!(zero_pad.left, 0.0);
    /// assert_eq!(zero_pad.right, 0.0);
    /// assert_eq!(zero_pad.top, 0.0);
    /// assert_eq!(zero_pad.bottom, 0.0);
    /// ```
    pub const fn zero() -> Self {
        Self::all(0.0)
    }

    const fn to_valid(x: f32) -> f32 {
        if x.is_nan() || x < 0.0 { 0.0 } else { x }
    }
}

/// Border property for UI components
///
/// # Notes
///
/// More properties (radius, dot-lined ...) will be supported in future release
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)] // Memory layout consistency
pub struct Border {
    /// Border width
    pub width: f32,

    /// Border color
    pub color: Color,
}

impl Border {
    /// Create Border with given width and color
    ///
    /// # Notes
    ///
    /// - width with `NaN` or negative value will be set to 0.0
    /// - More properties (radius, dot-lined ...) will be supported in future release
    ///
    /// # Arguments
    ///
    /// * `width` - Border line width
    /// * `color` - Border line color
    ///
    /// # Returns
    ///
    /// Returns Border { width: width, color: color }
    ///
    /// # Examples
    /// ```
    /// use ho_gui::style::Border;
    /// use ho_gui::color::Color;
    ///
    /// let valid_border = Border::new(1.0, Color::BLACK);
    /// let negative_border = Border::new(-1.0, Color::from_hex(0x00_FF_00));
    /// let nan_border = Border::new(f32::NAN, Color::from_hex_str("#FF00FF00").unwrap());
    ///
    /// // valid width should remain unchanged
    /// assert_eq!(
    ///     (valid_border.width, valid_border.color),
    ///     (1.0, Color::BLACK),
    /// );
    ///
    /// // negative width should clamp to 0.0
    /// assert_eq!(
    ///     (negative_border.width, negative_border.color),
    ///     (0.0, Color::from_hex(0x00_FF_00)),
    /// );
    ///
    /// // nan width should clamp to 0.0
    /// assert_eq!(
    ///     (nan_border.width, nan_border.color),
    ///     (0.0, Color::from_hex_str("#FF00FF00").unwrap()),
    /// );
    /// ```
    pub const fn new(width: f32, color: Color) -> Self {
        let width = Self::to_valid(width);
        Self { width, color }
    }

    /// Create empty, transparent border
    ///
    /// # Returns
    ///
    /// Returns Border { width: 0.0, color: Color::TRANSPARENT }
    ///
    /// # Examples
    /// ```
    /// use ho_gui::style::Border;
    /// use ho_gui::color::Color;
    ///
    /// let empty_border = Border::none();
    ///
    /// assert_eq!(
    ///     (empty_border.width, empty_border.color),
    ///     (0.0, Color::TRANSPARENT),
    /// );
    /// ```
    pub const fn none() -> Self {
        Self::new(0.0, Color::TRANSPARENT)
    }

    /// Create Border with given width and color
    ///
    /// # Notes
    ///
    /// - It behaves just as same as Border::new()
    /// - Width with `NaN` or negative value will be set to 0.0
    /// - More properties (radius, dot-lined ...) will be supported in future release
    ///
    /// # Arguments
    ///
    /// * `width` - Border line width
    /// * `color` - Border line color
    ///
    /// # Returns
    ///
    /// Returns Border { width: width, color: color }
    ///
    /// # Examples
    /// ```
    /// use ho_gui::style::Border;
    /// use ho_gui::color::Color;
    ///
    /// let valid_border = Border::solid(1.0, Color::BLACK);
    /// let negative_border = Border::solid(-1.0, Color::from_hex(0x00_FF_00));
    /// let nan_border = Border::solid(f32::NAN, Color::from_hex_str("#FF00FF00").unwrap());
    ///
    /// // valid width should remain unchanged
    /// assert_eq!(
    ///     (valid_border.width, valid_border.color),
    ///     (1.0, Color::BLACK),
    /// );
    ///
    /// // negative width should clamp to 0.0
    /// assert_eq!(
    ///     (negative_border.width, negative_border.color),
    ///     (0.0, Color::from_hex(0x00_FF_00)),
    /// );
    ///
    /// // nan width should clamp to 0.0
    /// assert_eq!(
    ///     (nan_border.width, nan_border.color),
    ///     (0.0, Color::from_hex_str("#FF00FF00").unwrap()),
    /// );
    /// ```
    pub const fn solid(width: f32, color: Color) -> Self {
        Self::new(width, color)
    }

    const fn to_valid(x: f32) -> f32 {
        if x.is_nan() || x < 0.0 { 0.0 } else { x }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod padding {
        use super::*;

        #[test]
        fn test_new() {
            let pad = Padding::new(f32::EPSILON, f32::INFINITY, -f32::EPSILON, f32::NAN);

            assert_eq!(
                pad.left,
                f32::EPSILON,
                "Valid value should remain unchanged. value: {}",
                pad.left
            );

            assert_eq!(
                pad.right,
                f32::INFINITY,
                "Positive infinity should remain unchanged. value: {}",
                pad.right
            );

            assert_eq!(
                pad.top, 0.0,
                "Negative value should set to 0.0. value: {}",
                pad.top
            );

            assert_eq!(
                pad.bottom, 0.0,
                "NaN should set to 0.0. value: {}",
                pad.bottom
            );
        }

        #[test]
        fn test_all() {
            let valid = Padding::all(f32::EPSILON);
            let infinity = Padding::all(f32::INFINITY);
            let negative = Padding::all(-f32::EPSILON);
            let nan = Padding::all(f32::NAN);

            assert_eq!(
                (valid.left, valid.right, valid.top, valid.bottom),
                (f32::EPSILON, f32::EPSILON, f32::EPSILON, f32::EPSILON),
                "Valid value should remain unchanged. value: {:?}",
                valid,
            );

            assert_eq!(
                (infinity.left, infinity.right, infinity.top, infinity.bottom),
                (f32::INFINITY, f32::INFINITY, f32::INFINITY, f32::INFINITY),
                "Positive infinity should remain unchanged. value: {:?}",
                infinity,
            );

            assert_eq!(
                (negative.left, negative.right, negative.top, negative.bottom),
                (0.0, 0.0, 0.0, 0.0),
                "Negative value should set to 0.0. value: {:?}",
                negative,
            );

            assert_eq!(
                (nan.left, nan.right, nan.top, nan.bottom),
                (0.0, 0.0, 0.0, 0.0),
                "NaN should set to 0.0. value: {:?}",
                nan,
            );
        }

        #[test]
        fn test_horizontal() {
            let valid = Padding::horizontal(f32::EPSILON);
            let infinity = Padding::horizontal(f32::INFINITY);
            let negative = Padding::horizontal(-f32::EPSILON);
            let nan = Padding::horizontal(f32::NAN);

            // check if horizontal values are set as specified
            assert_eq!(
                (valid.left, valid.right),
                (f32::EPSILON, f32::EPSILON),
                "Valid value should remain unchanged. value: {:?}",
                valid,
            );

            assert_eq!(
                (infinity.left, infinity.right),
                (f32::INFINITY, f32::INFINITY),
                "Positive infinity should remain unchanged. value: {:?}",
                infinity,
            );

            assert_eq!(
                (negative.left, negative.right),
                (0.0, 0.0),
                "Negative value should set to 0.0. value: {:?}",
                negative,
            );

            assert_eq!(
                (nan.left, nan.right),
                (0.0, 0.0),
                "NaN should set to 0.0. value: {:?}",
                nan,
            );

            // check if vertical values are set to 0.0
            [valid, infinity, negative, nan]
                .iter()
                .enumerate()
                .for_each(|(i, pad)| {
                    assert_eq!(
                        (pad.top, pad.bottom),
                        (0.0, 0.0),
                        "Vertical paddings should set to 0.0. index: {}, value: {:?}",
                        i,
                        pad,
                    );
                });
        }

        #[test]
        fn test_vertical() {
            let valid = Padding::vertical(f32::EPSILON);
            let infinity = Padding::vertical(f32::INFINITY);
            let negative = Padding::vertical(-f32::EPSILON);
            let nan = Padding::vertical(f32::NAN);

            // check if vertical values are set as specified
            assert_eq!(
                (valid.top, valid.bottom),
                (f32::EPSILON, f32::EPSILON),
                "Valid value should remain unchanged. value: {:?}",
                valid,
            );

            assert_eq!(
                (infinity.top, infinity.bottom),
                (f32::INFINITY, f32::INFINITY),
                "Positive infinity should remain unchanged. value: {:?}",
                infinity,
            );

            assert_eq!(
                (negative.top, negative.bottom),
                (0.0, 0.0),
                "Negative value should set to 0.0. value: {:?}",
                negative,
            );

            assert_eq!(
                (nan.top, nan.bottom),
                (0.0, 0.0),
                "NaN should set to 0.0. value: {:?}",
                nan,
            );

            // check if horizontal values are set to 0.0
            [valid, infinity, negative, nan]
                .iter()
                .enumerate()
                .for_each(|(i, pad)| {
                    assert_eq!(
                        (pad.left, pad.right),
                        (0.0, 0.0),
                        "Horizontal paddings should set to 0.0. index: {}, value: {:?}",
                        i,
                        pad,
                    );
                });
        }

        #[test]
        fn test_symmetric() {
            let valid = Padding::symmetric(f32::EPSILON, 1.0 - f32::EPSILON);
            let infinity = Padding::symmetric(f32::INFINITY, f32::INFINITY);
            let negative = Padding::symmetric(-f32::EPSILON, -f32::EPSILON);
            let nan = Padding::symmetric(f32::NAN, f32::NAN);

            // check if vertical values are set as specified
            assert_eq!(
                (valid.left, valid.right, valid.top, valid.bottom),
                (
                    f32::EPSILON,
                    f32::EPSILON,
                    1.0 - f32::EPSILON,
                    1.0 - f32::EPSILON
                ),
                "Valid value should remain unchanged. value: {:?}",
                valid,
            );

            assert_eq!(
                (infinity.left, infinity.right, infinity.top, infinity.bottom),
                (f32::INFINITY, f32::INFINITY, f32::INFINITY, f32::INFINITY),
                "Positive infinity should remain unchanged. value: {:?}",
                infinity,
            );

            assert_eq!(
                (negative.left, negative.right, negative.top, negative.bottom),
                (0.0, 0.0, 0.0, 0.0),
                "Negative value should set to 0.0. value: {:?}",
                negative,
            );

            assert_eq!(
                (nan.left, nan.right, nan.top, nan.bottom),
                (0.0, 0.0, 0.0, 0.0),
                "NaN should set to 0.0. value: {:?}",
                nan,
            );
        }

        #[test]
        fn test_zero() {
            let pad = Padding::zero();
            assert_eq!(
                (pad.left, pad.right, pad.top, pad.bottom),
                (0.0, 0.0, 0.0, 0.0),
                "All sides should set to 0.0. value: {:?}",
                pad,
            );
        }

        #[test]
        fn test_const_functions() {
            // const fn이 컴파일 타임에 동작하는지 확인
            const CONST_PAD: Padding = Padding::new(1.0, 2.0, 3.0, 4.0);
            const CONST_ALL: Padding = Padding::all(5.0);

            assert_eq!(CONST_PAD.left, 1.0);
            assert_eq!(CONST_ALL.left, 5.0);
        }
    }

    mod border {
        use super::*;

        #[test]
        fn test_new_valid_and_invalid_width() {
            let valid = Border::new(1.0, Color::BLACK);
            let invalid_negative = Border::new(-1.0, Color::TRANSPARENT);
            let invalid_nan = Border::new(f32::NAN, Color::BLUE);

            assert_eq!(
                valid.width, 1.0,
                "Valid width should remain unchanged. width: {}",
                valid.width
            );

            assert_eq!(
                valid.color,
                Color::BLACK,
                "Color should remain unchanged. color: {:?}",
                valid.color
            );

            assert_eq!(
                invalid_negative.width, 0.0,
                "Negative width should be set to 0.0. width: {}",
                invalid_negative.width
            );

            assert_eq!(
                invalid_negative.color,
                Color::TRANSPARENT,
                "Color should remain unchanged. color: {:?}",
                invalid_negative.color
            );

            assert_eq!(
                invalid_nan.width, 0.0,
                "NaN width should be set to 0.0. width: {}",
                invalid_nan.width
            );

            assert_eq!(
                invalid_nan.color,
                Color::BLUE,
                "Color should remain unchanged. color: {:?}",
                invalid_nan.color
            );
        }

        #[test]
        fn test_none() {
            let none = Border::none();

            assert_eq!(
                none.width, 0.0,
                "Border None should have width 0.0. width: {}",
                none.width
            );
            assert_eq!(
                none.color,
                Color::TRANSPARENT,
                "Border None should be transparent. color: {:?}",
                none.color
            );
        }

        #[test]
        fn test_solid_valid_and_invalid_width() {
            let valid = Border::solid(1.0, Color::BLACK);
            let invalid_negative = Border::solid(-1.0, Color::TRANSPARENT);
            let invalid_nan = Border::solid(f32::NAN, Color::BLUE);

            assert_eq!(
                valid.width, 1.0,
                "Valid width should remain unchanged. width: {}",
                valid.width
            );

            assert_eq!(
                valid.color,
                Color::BLACK,
                "Color should remain unchanged. color: {:?}",
                valid.color
            );

            assert_eq!(
                invalid_negative.width, 0.0,
                "Negative width should be set to 0.0. width: {}",
                invalid_negative.width
            );

            assert_eq!(
                invalid_negative.color,
                Color::TRANSPARENT,
                "Color should remain unchanged. color: {:?}",
                invalid_negative.color
            );

            assert_eq!(
                invalid_nan.width, 0.0,
                "NaN width should be set to 0.0. width: {}",
                invalid_nan.width
            );

            assert_eq!(
                invalid_nan.color,
                Color::BLUE,
                "Color should remain unchanged. color: {:?}",
                invalid_nan.color
            );
        }

        #[test]
        fn test_const_functions() {
            // const fn이 컴파일 타임에 동작하는지 확인
            const CONST_BORDER: Border = Border::new(2.0, Color::RED);
            const CONST_NONE: Border = Border::none();
            const CONST_SOLID: Border = Border::solid(3.0, Color::BLUE);

            assert_eq!(CONST_BORDER.width, 2.0);
            assert_eq!(CONST_BORDER.color, Color::RED);
            assert_eq!(CONST_NONE.width, 0.0);
            assert_eq!(CONST_NONE.color, Color::TRANSPARENT);
            assert_eq!(CONST_SOLID.width, 3.0);
            assert_eq!(CONST_SOLID.color, Color::BLUE);
        }
    }
}

#[cfg(test)]
mod bench_tests {
    use super::*;

    // Simple benchmark-style tests (for actual benchmarking, use criterion crate)
    #[test]
    fn test_padding_construction_performance() {
        let start = std::time::Instant::now();
        const ITERATIONS: usize = 100_000;

        for i in 0..ITERATIONS {
            let value = (i % 1000) as f32 / 10.0;
            let _padding = Padding::new(value, value * 1.1, value * 1.2, value * 1.3);
        }

        let elapsed = start.elapsed();
        let ns_per_op = elapsed.as_nanos() as f64 / ITERATIONS as f64;

        println!("Padding::new() performance: {:.2}ns per operation", ns_per_op);
        assert!(
            ns_per_op < 50.0,
            "Performance regression: {}ns > 50ns",
            ns_per_op
        );
    }

    #[test]
    fn test_padding_convenience_constructors_performance() {
        let start = std::time::Instant::now();
        const ITERATIONS: usize = 100_000;

        for i in 0..ITERATIONS {
            let value = (i % 1000) as f32 / 10.0;
            let _all = Padding::all(value);
            let _horizontal = Padding::horizontal(value);
            let _vertical = Padding::vertical(value);
            let _symmetric = Padding::symmetric(value, value * 2.0);
        }

        let elapsed = start.elapsed();
        let ns_per_op = elapsed.as_nanos() as f64 / (ITERATIONS * 4) as f64;

        println!("Padding convenience constructors performance: {:.2}ns per operation", ns_per_op);
        assert!(
            ns_per_op < 50.0,
            "Performance regression: {}ns > 50ns",
            ns_per_op
        );
    }

    #[test]
    fn test_border_construction_performance() {
        let start = std::time::Instant::now();
        const ITERATIONS: usize = 100_000;

        for i in 0..ITERATIONS {
            let width = (i % 100) as f32 / 10.0;
            let color = Color::rgb((i % 256) as f32 / 255.0, 0.5, 0.8);
            let _border = Border::new(width, color);
        }

        let elapsed = start.elapsed();
        let ns_per_op = elapsed.as_nanos() as f64 / ITERATIONS as f64;

        println!("Border::new() performance: {:.2}ns per operation", ns_per_op);
        assert!(
            ns_per_op < 50.0,
            "Performance regression: {}ns > 50ns",
            ns_per_op
        );
    }

    #[test]
    fn test_border_convenience_constructors_performance() {
        let start = std::time::Instant::now();
        const ITERATIONS: usize = 100_000;

        for i in 0..ITERATIONS {
            let width = (i % 100) as f32 / 10.0;
            let color = Color::from_hex((i % 0xFFFFFF) as u32);
            let _solid = Border::solid(width, color);
            let _none = Border::none();
        }

        let elapsed = start.elapsed();
        let ns_per_op = elapsed.as_nanos() as f64 / (ITERATIONS * 2) as f64;

        println!("Border convenience constructors performance: {:.2}ns per operation", ns_per_op);
        assert!(
            ns_per_op < 50.0,
            "Performance regression: {}ns > 50ns",
            ns_per_op
        );
    }

    #[test]
    fn test_memory_layout_verification() {
        // Verify memory layout for GPU compatibility
        assert_eq!(std::mem::size_of::<Padding>(), 16);
        assert_eq!(std::mem::align_of::<Padding>(), 4);

        // Border has Color (16 bytes, 16-byte aligned) + f32 (4 bytes) + padding (12 bytes)
        // Total: 32 bytes due to Color's 16-byte alignment requirement
        assert_eq!(std::mem::size_of::<Border>(), 32);
        assert_eq!(std::mem::align_of::<Border>(), 16);

        println!("Padding: {} bytes, {} byte alignment", 
                std::mem::size_of::<Padding>(), 
                std::mem::align_of::<Padding>());
        println!("Border: {} bytes, {} byte alignment", 
                std::mem::size_of::<Border>(), 
                std::mem::align_of::<Border>());
    }

    #[test]
    fn test_const_evaluation_performance() {
        // Verify const functions work at compile time (zero runtime cost)
        const _CONST_PADDING: Padding = Padding::symmetric(16.0, 8.0);
        const _CONST_BORDER: Border = Border::solid(2.0, Color::BLACK);
        
        // These should have zero runtime cost
        let start = std::time::Instant::now();
        const ITERATIONS: usize = 100_000;

        for _ in 0..ITERATIONS {
            let _padding = _CONST_PADDING;
            let _border = _CONST_BORDER;
        }

        let elapsed = start.elapsed();
        let ns_per_op = elapsed.as_nanos() as f64 / (ITERATIONS * 2) as f64;

        println!("Const value access performance: {:.2}ns per operation", ns_per_op);
        assert!(
            ns_per_op < 10.0,
            "Const evaluation should be near-zero cost: {}ns > 10ns",
            ns_per_op
        );
    }
}
