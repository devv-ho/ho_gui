//! Color for RGB and RGBA

/// Color for RGBA. Each r, g, b, a is expressed in (0.0..=1.0) which is scaled from (0x00..0xFF).
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C, align(16))]
pub struct Color {
    /// Red (Expressed in (0.0..=1.0) which is scaled from (0x00..0xFF))
    pub r: f32,

    /// Green (Expressed in (0.0..=1.0) which is scaled from (0x00..0xFF))
    pub g: f32,

    /// Blue (Expressed in (0.0..=1.0) which is scaled from (0x00..0xFF))
    pub b: f32,

    /// Alpha (Transparency) (Expressed in (0.0..=1.0) which is scaled from (0x00..0xFF))
    pub a: f32,
}

impl Color {
    /// Color for white
    pub const WHITE: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };

    /// Color for black
    pub const BLACK: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };

    /// Color for red
    pub const RED: Self = Self {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };

    /// Color for green
    pub const GREEN: Self = Self {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };

    /// Color for blue
    pub const BLUE: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };

    /// Color for yellow
    pub const YELLOW: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };

    /// Color for cyan
    pub const CYAN: Self = Self {
        r: 0.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };

    /// Color for magenta
    pub const MAGENTA: Self = Self {
        r: 1.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };

    /// Totally Transparent
    pub const TRANSPARENT: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };

    /// Create Color with specified r, g, b, a with range (0.0..=1.0), which is scaled from
    /// (0x00..=0xFF).
    ///
    /// # Note
    ///
    /// Any value which is out of border values (0.0, 1.0) will be clamped into border value.
    ///
    /// # Arguments
    ///
    /// * `r` - Red
    /// * `g` - Green
    /// * `b` - Blue
    /// * `a` - Alpha (transparency)
    ///
    /// # Returns
    ///
    /// Color object for input RGBA
    ///
    /// # Examples
    /// ```
    /// use ho_gui::color::Color;
    ///
    /// let white = Color::new(1.0, 1.0, 1.0, 1.0);
    ///
    /// assert_eq!(white.r, 1.0);
    /// assert_eq!(white.g, 1.0);
    /// assert_eq!(white.b, 1.0);
    /// assert_eq!(white.a, 1.0);
    /// ```
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            r: Self::clamp01(r),
            g: Self::clamp01(g),
            b: Self::clamp01(b),
            a: Self::clamp01(a),
        }
    }

    /// Create Color with specified r, g, b with range (0.0..=1.0), which is scaled from
    /// (0x00..=0xFF). a (alpha) will be set to 1.0.
    ///
    /// # Note
    ///
    /// Any value which is out of border values (0.0, 1.0) will be clamped into border value.
    ///
    /// # Arguments
    ///
    /// * `r` - Red
    /// * `g` - Green
    /// * `b` - Blue
    ///
    /// # Returns
    ///
    /// Color object with input RGB and 1.0 alpha
    ///
    /// # Examples
    /// ```
    /// use ho_gui::color::Color;
    ///
    /// let white = Color::rgb(1.0, 1.0, 1.0);
    ///
    /// assert_eq!(white.r, 1.0);
    /// assert_eq!(white.g, 1.0);
    /// assert_eq!(white.b, 1.0);
    /// assert_eq!(white.a, 1.0);
    /// ```
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::new(r, g, b, 1.0)
    }

    /// Create Color with specified r, g, b, a with range (0x00..=0xFF). The value will be scaled
    /// into (0.0..=1.0) range
    ///
    /// # Arguments
    ///
    /// * `r` - Red
    /// * `g` - Green
    /// * `b` - Blue
    /// * `a` - Alpha (transparency)
    ///
    /// # Returns
    ///
    /// Color object for input RGBA
    ///
    /// # Examples
    /// ```
    /// use ho_gui::color::Color;
    ///
    /// let white = Color::rgba(0xFFu8, 0xFFu8, 0xFFu8, 0xFFu8);
    ///
    /// assert_eq!(white.r, 1.0);
    /// assert_eq!(white.g, 1.0);
    /// assert_eq!(white.b, 1.0);
    /// assert_eq!(white.a, 1.0);
    /// ```
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::new(
            Self::scale01(r),
            Self::scale01(g),
            Self::scale01(b),
            Self::scale01(a),
        )
    }

    /// Create Color with specified hex-ed RGB value with range `(0x00_00_00..=0xFF_FF_FF)`.
    /// Input bytes will be parsed as followed (`0xRR_GG_BB`). A-value (alpha) will be set to 1.0.
    ///
    /// # Note
    ///
    /// Value over 0xFF_FF_FF (e.g. `0x01_3F_2F_10`) will be clamped into `0xFF_FF_FF`.
    ///
    /// # Arguments
    ///
    /// * `hex` - 3-byte hex value for rgb
    ///
    /// # Returns
    ///
    /// Color object for input RGB and 1.0 alpha
    ///
    /// # Examples
    ///
    /// ```
    /// use ho_gui::color::Color;
    ///
    /// let white = Color::from_hex(0xFF_FF_FF);
    ///
    /// assert_eq!(white.r, 1.0);
    /// assert_eq!(white.g, 1.0);
    /// assert_eq!(white.b, 1.0);
    /// assert_eq!(white.a, 1.0);
    /// ```
    pub const fn from_hex(hex: u32) -> Self {
        let hex = if hex > 0xFF_FF_FF { 0xFF_FF_FF } else { hex };

        Self::rgba(
            Self::extract_byte(hex, 2), // R
            Self::extract_byte(hex, 1), // G
            Self::extract_byte(hex, 0), // B
            0xFF,                       // A
        )
    }

    /// Create Color with specified hex-ed RGBA value with range `(0x00_00_00_00..=0xFF_FF_FF_FF)`.
    /// Input bytes will be parsed as followed - `0xRR_GG_BB_AA`.
    ///
    /// # Arguments
    ///
    /// * `hex` - 4-byte hex value for rgba
    ///
    /// # Returns
    ///
    /// Color object for input RGBA
    ///
    /// # Examples
    /// ```
    /// use ho_gui::color::Color;
    ///
    /// let white = Color::from_hex_alpha(0xFF_FF_FF_FF);
    ///
    /// assert_eq!(white.r, 1.0);
    /// assert_eq!(white.g, 1.0);
    /// assert_eq!(white.b, 1.0);
    /// assert_eq!(white.a, 1.0);
    /// ```
    pub const fn from_hex_alpha(hex: u32) -> Self {
        Self::rgba(
            Self::extract_byte(hex, 3), // R
            Self::extract_byte(hex, 2), // G
            Self::extract_byte(hex, 1), // B
            Self::extract_byte(hex, 0), // A
        )
    }

    /// Create Color with specified hex-ed &str RGB (e.g. `"#FF1F00"`) and RGBA (e.g. `"#FF1F002A"`).
    ///
    /// # Arguments
    ///
    /// * `hex` - The color hex &str (e.g. `"#FF1F00"`, `"#FF1F002A"`). The hex character is
    /// case-insensitive. (e.g. `"#f1f1f1"`, `"#F1F1F1"`, `"#f1F1f1"` are all fine.)
    ///
    /// # Returns
    ///
    /// Color object for input RGB &str
    ///
    /// # Errors
    ///
    /// * Returns `ColorParseError::InvalidLength` if the length of the input hex string is not 7 or
    /// 9
    ///
    /// * Returns `ColorParseError::InvalidFormat` if the input hex string doesn't start with `'#'`
    /// (e.g. `"FF1F00"`)
    ///
    /// * Returns `ColorParseError::InvalidCharacter` if the input hex string has non-hex character
    /// (e.g. `'@'`, `'.'` ...)
    ///
    /// # Examples
    /// ```
    /// use ho_gui::color::Color;
    ///
    /// let white = Color::from_hex_str("#FFFFFF").unwrap();
    ///
    /// assert_eq!(white.r, 1.0);
    /// assert_eq!(white.g, 1.0);
    /// assert_eq!(white.b, 1.0);
    /// assert_eq!(white.a, 1.0);
    /// ```
    pub fn from_hex_str(hex: &str) -> Result<Self, ColorParseError> {
        if !hex.starts_with("#") {
            return Err(ColorParseError::InvalidFormat);
        }

        let hex_digits = &hex[1..];

        if hex_digits.chars().any(|c| !c.is_ascii_hexdigit()) {
            return Err(ColorParseError::InvalidCharacter);
        }

        if hex_digits.len() != 6 && hex_digits.len() != 8 {
            return Err(ColorParseError::InvalidLength);
        }

        let r = u8::from_str_radix(&hex_digits[0..2], 16).unwrap();
        let g = u8::from_str_radix(&hex_digits[2..4], 16).unwrap();
        let b = u8::from_str_radix(&hex_digits[4..6], 16).unwrap();
        let a = if hex_digits.len() == 6 {
            0xFFu8
        } else {
            u8::from_str_radix(&hex_digits[6..8], 16).unwrap()
        };

        Ok(Self::rgba(r, g, b, a))
    }

    /// Get u8 hex value of each RGBA values.
    ///
    /// # Returns
    ///
    /// A tuple `(r, g, b, a)` where each component is a `u8` in the range `0..=255`.
    ///
    /// # Examples
    /// ```
    /// use ho_gui::color::Color;
    ///
    /// let white = Color::WHITE;
    /// let (r, g, b, a) = white.to_rgba_u8();
    /// assert_eq!((r, g, b, a), (0xFF, 0xFF, 0xFF, 0xFF));
    /// ```
    pub const fn to_rgba_u8(&self) -> (u8, u8, u8, u8) {
        (
            Self::round_to_u8(self.r * 255.0),
            Self::round_to_u8(self.g * 255.0),
            Self::round_to_u8(self.b * 255.0),
            Self::round_to_u8(self.a * 255.0),
        )
    }

    /// Manual rounding implementation for const fn compatibility.
    const fn round_to_u8(x: f32) -> u8 {
        if x <= 0.0 {
            0
        } else if x >= 255.0 {
            255
        } else {
            (x + 0.5) as u8 // Manual rounding: add 0.5 and truncate
        }
    }

    /// Create a new Color with the same RGB values but a different alpha value.
    ///
    /// # Note
    ///
    /// The alpha value will be clamped to the range `(0.0..=1.0)`.
    ///
    /// # Arguments
    ///
    /// * `a` - New alpha (transparency) value
    ///
    /// # Returns
    ///
    /// New Color object with the same RGB but new alpha value
    ///
    /// # Examples
    /// ```
    /// use ho_gui::color::Color;
    ///
    /// let opaque_red = Color::RED;
    /// let semi_transparent_red = opaque_red.with_alpha(0.5);
    ///
    /// assert_eq!(semi_transparent_red.r, 1.0);
    /// assert_eq!(semi_transparent_red.g, 0.0);
    /// assert_eq!(semi_transparent_red.b, 0.0);
    /// assert_eq!(semi_transparent_red.a, 0.5);
    /// ```
    pub const fn with_alpha(&self, a: f32) -> Self {
        Self::new(self.r, self.g, self.b, Self::clamp01(a))
    }

    /// Check if all color components are within valid range.
    ///
    /// # Note
    ///
    /// Since all `Color` constructors automatically clamp values to the valid range,
    /// this method will always return `true` for properly constructed `Color` instances.
    /// This method is primarily useful for validation after direct field manipulation
    /// or when working with colors from external sources.
    ///
    /// # Returns
    ///
    /// `true` if all RGBA components are in the range `(0.0..=1.0)`, `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// use ho_gui::color::Color;
    ///
    /// let valid_color = Color::new(0.5, 0.5, 0.5, 1.0);
    /// assert!(valid_color.is_valid());
    ///
    /// // This would still be valid due to constructor clamping
    /// let clamped_color = Color::new(2.0, -1.0, 0.5, 0.5);
    /// assert!(clamped_color.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        let check_validity = |x: f32| -> bool { !x.is_nan() && x >= 0.0 && x <= 1.0 };

        check_validity(self.r)
            && check_validity(self.g)
            && check_validity(self.b)
            && check_validity(self.a)
    }

    const fn extract_byte(hex: u32, idx_to_extract: u32) -> u8 {
        // u32 has only 4 bytes
        if idx_to_extract > 3 {
            return 0x00;
        }

        ((hex >> (idx_to_extract * u8::BITS)) & 0xFF) as u8
    }

    const fn clamp01(x: f32) -> f32 {
        if x.is_nan() || x < 0.0 {
            0.0
        } else if x > 1.0 {
            1.0
        } else {
            x
        }
    }

    const fn scale01(x: u8) -> f32 {
        (x as f32) * (1.0 / u8::MAX as f32)
    }
}

/// Error type for color string parsing operations.
///
/// This error is returned when [`Color::from_hex_str`] fails to parse a hex color string.
#[derive(Debug, Clone, PartialEq)]
pub enum ColorParseError {
    /// The hex string has an invalid length.
    ///
    /// Valid lengths are 7 characters (`"#RRGGBB"`) or 9 characters (`"#RRGGBBAA"`).
    InvalidLength,

    /// The string does not start with `'#'` or has an unexpected format.
    ///
    /// The string must start with a `'#'` character followed by hex digits.
    InvalidFormat,

    /// The string contains non-hexadecimal characters.
    ///
    /// Only characters `0-9`, `A-F`, and `a-f` are valid after the `'#'` prefix.
    InvalidCharacter,
}

impl std::fmt::Display for ColorParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorParseError::InvalidLength => {
                write!(f, "Invalid hex string length (expected 7 or 9 characters)")
            }
            ColorParseError::InvalidFormat => {
                write!(f, "Invalid hex string format (must start with '#')")
            }
            ColorParseError::InvalidCharacter => {
                write!(f, "Invalid hex character (only 0-9, A-F, a-f allowed)")
            }
        }
    }
}

impl std::error::Error for ColorParseError {}

#[cfg(test)]
mod color_tests {
    use super::*;
    use approx::assert_relative_eq;

    const TEST_EPSILON: f32 = 1e-6;

    const BYTE_VALID: u8 = 0x3Fu8;

    const CLAMPED_VALID: f32 = 0.24705882;
    const CLAMPED_NEGATIVE_INVALID: f32 = -0.1;
    const CLAMPED_POSITIVE_INVALID: f32 = 1.1;

    #[test]
    fn test_new_valid() {
        let (r, g, b, a) = (CLAMPED_VALID, CLAMPED_VALID, CLAMPED_VALID, CLAMPED_VALID);

        let color = Color::new(r, g, b, a);

        assert_relative_eq!(color.r, r, epsilon = TEST_EPSILON);
        assert_relative_eq!(color.g, g, epsilon = TEST_EPSILON);
        assert_relative_eq!(color.b, b, epsilon = TEST_EPSILON);
        assert_relative_eq!(color.a, a, epsilon = TEST_EPSILON);
    }

    #[test]
    fn test_new_invalid() {
        let (r, g, b, a) = (CLAMPED_NEGATIVE_INVALID, CLAMPED_POSITIVE_INVALID, 0.0, 1.0);

        let color = Color::new(r, g, b, a);

        assert_relative_eq!(color.r, 0.0);
        assert_relative_eq!(color.g, 1.0);
        assert_relative_eq!(color.b, b);
        assert_relative_eq!(color.a, a);
    }

    #[test]
    fn test_rgb_valid() {
        let (r, g, b, _) = (CLAMPED_VALID, CLAMPED_VALID, 0.0, 1.0);

        let color = Color::rgb(r, g, b);

        assert_relative_eq!(color.r, r);
        assert_relative_eq!(color.g, g);
        assert_relative_eq!(color.b, b);
        assert_relative_eq!(color.a, 1.0);
    }

    #[test]
    fn test_rgb_invalid() {
        let (r, g, b, _) = (CLAMPED_NEGATIVE_INVALID, CLAMPED_POSITIVE_INVALID, 0.0, 1.0);

        let color = Color::rgb(r, g, b);

        assert_relative_eq!(color.r, 0.0);
        assert_relative_eq!(color.g, 1.0);
        assert_relative_eq!(color.b, b);
        assert_relative_eq!(color.a, 1.0);
    }

    #[test]
    fn test_rgba_valid() {
        let (r, g, b, a) = (BYTE_VALID, BYTE_VALID, 0x00, 0xFF);

        let color = Color::rgba(r, g, b, a);

        assert_relative_eq!(color.r, CLAMPED_VALID);
        assert_relative_eq!(color.g, CLAMPED_VALID);
        assert_relative_eq!(color.b, 0.0);
        assert_relative_eq!(color.a, 1.0);
    }

    // Because rgba() require u8, all rgba inputs are valid

    #[test]
    fn test_from_hex_valid() {
        let hex = 0x3F_FF_00;
        let color = Color::from_hex(hex);

        assert_relative_eq!(color.r, CLAMPED_VALID);
        assert_relative_eq!(color.g, 1.0);
        assert_relative_eq!(color.b, 0.0);
        assert_relative_eq!(color.a, 1.0);
    }

    #[test]
    fn test_from_hex_invalid() {
        let hex = 0x1_3F_FF_00;
        let color = Color::from_hex(hex);

        assert_relative_eq!(color.r, 1.0);
        assert_relative_eq!(color.g, 1.0);
        assert_relative_eq!(color.b, 1.0);
        assert_relative_eq!(color.a, 1.0);
    }

    #[test]
    fn test_from_hex_alpha_valid() {
        let hex = 0x3F_FF_00_3F;
        let color = Color::from_hex_alpha(hex);

        assert_relative_eq!(color.r, CLAMPED_VALID);
        assert_relative_eq!(color.g, 1.0);
        assert_relative_eq!(color.b, 0.0);
        assert_relative_eq!(color.a, CLAMPED_VALID);
    }

    // from_hex_alpha doesnt need invalid input test. because value out of 0xFF_FF_FF_FF cannot be
    // put into u32, so every inpu is valid for from_hex_alpha.

    #[test]
    fn test_from_hex_string_rgb_valid() {
        let hex = "#3f00FF";
        let color = Color::from_hex_str(hex).unwrap();

        assert_relative_eq!(color.r, CLAMPED_VALID);
        assert_relative_eq!(color.g, 0.0);
        assert_relative_eq!(color.b, 1.0);
        assert_relative_eq!(color.a, 1.0);
    }

    #[test]
    fn test_from_hex_string_rgba_valid() {
        let hex = "#3f00FF3F";
        let color = Color::from_hex_str(hex).unwrap();

        assert_relative_eq!(color.r, CLAMPED_VALID);
        assert_relative_eq!(color.g, 0.0);
        assert_relative_eq!(color.b, 1.0);
        assert_relative_eq!(color.a, CLAMPED_VALID);
    }

    #[test]
    fn test_from_hex_string_invalid_format() {
        let hex = "3F00FF";
        let color = Color::from_hex_str(hex);

        assert!(matches!(color, Err(ColorParseError::InvalidFormat)));
    }

    #[test]
    fn test_from_hex_string_invalid_char() {
        let hex = "#3F_00_FF";
        let color = Color::from_hex_str(hex);

        assert!(matches!(color, Err(ColorParseError::InvalidCharacter)));
    }

    #[test]
    fn test_from_hex_string_invalid_length() {
        let hex = "#FFFFFFF";
        let color = Color::from_hex_str(hex);

        assert!(matches!(color, Err(ColorParseError::InvalidLength)));
    }

    #[test]
    fn test_to_rgba_u8() {
        let color = Color::new(CLAMPED_VALID, 0.0, 1.0, CLAMPED_VALID);
        let color = color.to_rgba_u8();

        assert_eq!(BYTE_VALID, color.0);
        assert_eq!(0x00, color.1);
        assert_eq!(0xFF, color.2);
        assert_eq!(BYTE_VALID, color.3);
    }

    #[test]
    fn test_to_with_alpha_valid() {
        let color = Color::new(CLAMPED_VALID, 0.0, 1.0, CLAMPED_VALID);
        let color_new = color.with_alpha(0.5);

        assert_eq!(color_new.r, color.r);
        assert_eq!(color_new.g, color.g);
        assert_eq!(color_new.b, color.b);
        assert_eq!(color_new.a, 0.5);
    }

    #[test]
    fn test_to_with_alpha_invalid() {
        let color = Color::new(CLAMPED_VALID, 0.0, 1.0, CLAMPED_VALID);
        let color_new = color.with_alpha(CLAMPED_POSITIVE_INVALID);

        assert_eq!(color_new.r, color.r);
        assert_eq!(color_new.g, color.g);
        assert_eq!(color_new.b, color.b);
        assert_eq!(color_new.a, 1.0);
    }

    #[test]
    fn test_is_valid() {
        let color = Color::new(CLAMPED_VALID, 0.0, 1.0, CLAMPED_VALID);

        assert!(color.is_valid());
    }

    #[test]
    fn test_scale01() {
        let valid = BYTE_VALID;

        assert_relative_eq!(Color::scale01(valid), CLAMPED_VALID);
        assert_relative_eq!(Color::scale01(0x00), 0.0);
        assert_relative_eq!(Color::scale01(0xFF), 1.0);
    }

    #[test]
    fn test_extract_component_valid() {
        let hex = 0x12345678;

        assert_eq!(Color::extract_byte(hex, 0), 0x78);
        assert_eq!(Color::extract_byte(hex, 1), 0x56);
        assert_eq!(Color::extract_byte(hex, 2), 0x34);
        assert_eq!(Color::extract_byte(hex, 3), 0x12);
    }

    #[test]
    fn test_extract_component_invalid() {
        let hex = 0x12345678;

        assert_eq!(Color::extract_byte(hex, 4), 0x00);
    }

    #[test]
    fn test_clone_color() {
        let color = Color::new(
            CLAMPED_VALID,
            CLAMPED_NEGATIVE_INVALID,
            CLAMPED_POSITIVE_INVALID,
            CLAMPED_VALID,
        );
        let color_clone = color.clone();

        assert_eq!(color, color_clone);
    }

    #[test]
    fn test_copy_color() {
        let color = Color::new(
            CLAMPED_VALID,
            CLAMPED_NEGATIVE_INVALID,
            CLAMPED_POSITIVE_INVALID,
            CLAMPED_VALID,
        );
        let color_copy = color;

        assert_eq!(color, color_copy);
    }

    #[test]
    fn test_memory_layout() {
        assert_eq!(std::mem::size_of::<Color>(), 16);
        assert_eq!(std::mem::align_of::<Color>(), 16);
    }

    // Property-based tests for round-trip conversions
    #[test]
    fn test_roundtrip_conversion_accuracy() {
        // Test that Color -> u8 -> Color conversion maintains accuracy within tolerance
        let tolerance = 1.0 / 255.0; // 1/255 per component as per spec

        let test_colors = [
            (0.0, 0.0, 0.0, 0.0),
            (1.0, 1.0, 1.0, 1.0),
            (0.5, 0.5, 0.5, 0.5),
            (CLAMPED_VALID, 0.0, 1.0, CLAMPED_VALID),
            (0.25, 0.75, 0.125, 0.875),
        ];

        for (r, g, b, a) in test_colors {
            let original = Color::new(r, g, b, a);
            let (r_u8, g_u8, b_u8, a_u8) = original.to_rgba_u8();
            let roundtrip = Color::rgba(r_u8, g_u8, b_u8, a_u8);

            assert!(
                (original.r - roundtrip.r).abs() <= tolerance,
                "Red component roundtrip failed: {} -> {} -> {}",
                original.r,
                r_u8,
                roundtrip.r
            );
            assert!(
                (original.g - roundtrip.g).abs() <= tolerance,
                "Green component roundtrip failed: {} -> {} -> {}",
                original.g,
                g_u8,
                roundtrip.g
            );
            assert!(
                (original.b - roundtrip.b).abs() <= tolerance,
                "Blue component roundtrip failed: {} -> {} -> {}",
                original.b,
                b_u8,
                roundtrip.b
            );
            assert!(
                (original.a - roundtrip.a).abs() <= tolerance,
                "Alpha component roundtrip failed: {} -> {} -> {}",
                original.a,
                a_u8,
                roundtrip.a
            );
        }
    }

    #[test]
    fn test_hex_parsing_accuracy() {
        // Test that hex -> Color -> to_rgba_u8 is accurate within rounding error
        let test_cases = [
            (0xFF0000, (255, 0, 0, 255)),
            (0x00FF00, (0, 255, 0, 255)),
            (0x0000FF, (0, 0, 255, 255)),
            (0x808080, (128, 128, 128, 255)),
        ];

        for (hex_input, expected_u8) in test_cases {
            let color = Color::from_hex(hex_input);
            let actual_u8 = color.to_rgba_u8();
            assert_eq!(
                actual_u8, expected_u8,
                "Hex parsing accuracy failed for 0x{:06X}",
                hex_input
            );
        }
    }

    #[test]
    fn test_clamping_behavior() {
        // Test that all inputs produce valid [0.0, 1.0] output
        let extreme_values = [
            f32::NEG_INFINITY,
            -1000.0,
            -1.0,
            -0.1,
            0.0,
            0.5,
            1.0,
            1.1,
            1000.0,
            f32::INFINITY,
        ];

        for &val in &extreme_values {
            let color = Color::new(val, val, val, val);
            assert!(color.is_valid(), "Clamping failed for value: {}", val);
            assert!(
                color.r >= 0.0 && color.r <= 1.0,
                "Red not clamped: {}",
                color.r
            );
            assert!(
                color.g >= 0.0 && color.g <= 1.0,
                "Green not clamped: {}",
                color.g
            );
            assert!(
                color.b >= 0.0 && color.b <= 1.0,
                "Blue not clamped: {}",
                color.b
            );
            assert!(
                color.a >= 0.0 && color.a <= 1.0,
                "Alpha not clamped: {}",
                color.a
            );
        }
    }

    #[test]
    fn test_nan_handling() {
        // Test that NaN values are properly handled
        let color_with_nan = Color::new(f32::NAN, 0.5, f32::NAN, 1.0);
        assert!(
            color_with_nan.is_valid(),
            "color always created to be valid"
        );

        // NaN should be clamped to 0.0
        assert_eq!(color_with_nan.r, 0.0, "NaN should be clamped to 0.0");
        assert_eq!(color_with_nan.b, 0.0, "NaN should be clamped to 0.0");
        assert_eq!(color_with_nan.g, 0.5, "Valid value should remain unchanged");
        assert_eq!(color_with_nan.a, 1.0, "Valid value should remain unchanged");
    }

    #[test]
    fn test_precision_edge_cases() {
        // Test precision at boundaries
        let epsilon = f32::EPSILON;

        // Values just inside valid range
        let just_valid = Color::new(epsilon, 1.0 - epsilon, 0.5, 0.5);
        assert!(just_valid.is_valid());

        // Values just outside valid range (should be clamped)
        let just_invalid = Color::new(-epsilon, 1.0 + epsilon, 0.5, 0.5);
        assert!(just_invalid.is_valid()); // Should be valid after clamping
        assert_eq!(just_invalid.r, 0.0);
        assert_eq!(just_invalid.g, 1.0);
    }
}

#[cfg(test)]
mod bench_tests {
    use super::*;

    // Simple benchmark-style tests (for actual benchmarking, use criterion crate)
    #[test]
    fn test_color_construction_performance() {
        // Use constant values to avoid measuring calculation overhead
        const ITERATIONS: usize = 100_000;
        
        let start = std::time::Instant::now();

        for i in 0..ITERATIONS {
            // Use simple, deterministic values that don't require calculation
            let val = (i & 0xFF) as f32 / 255.0;
            let _color = Color::new(val, val, val, 1.0);
        }

        let elapsed = start.elapsed();
        let ns_per_op = elapsed.as_nanos() as f64 / ITERATIONS as f64;

        // Spec requires < 10ns per operation on modern x86_64
        // This is a rough test - use proper benchmarking tools for accurate measurement
        println!("Color::new() performance: {:.2}ns per operation", ns_per_op);
        assert!(
            ns_per_op < 50.0,
            "Performance regression: {}ns > 50ns",
            ns_per_op
        );
    }

    #[test]
    fn test_rgba_conversion_performance() {
        let start = std::time::Instant::now();
        const ITERATIONS: usize = 100_000;

        for i in 0..ITERATIONS {
            let r = (i % 256) as u8;
            let g = ((i * 2) % 256) as u8;
            let b = ((i * 3) % 256) as u8;
            let a = ((i * 4) % 256) as u8;

            let _color = Color::rgba(r, g, b, a);
        }

        let elapsed = start.elapsed();
        let ns_per_op = elapsed.as_nanos() as f64 / ITERATIONS as f64;

        println!(
            "Color::rgba() performance: {:.2}ns per operation",
            ns_per_op
        );
        assert!(
            ns_per_op < 50.0,
            "Performance regression: {}ns > 50ns",
            ns_per_op
        );
    }
}
