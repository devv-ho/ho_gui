# Task 1.2.2: Implement Color and Style Types

**Priority**: HIGH  
**Status**: ASSIGNED  
**Developer**: Team Member  
**Phase**: 1.2 Core Abstractions  
**Estimated Time**: 2-3 hours  

## Objective
Create the color and basic styling types required for GUI theming and visual presentation.

## Technical Requirements

### File: `src/color.rs`
Implement the following types with complete functionality:

#### 1. Color Struct
```rust
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C, align(16))]  // GPU-compatible memory layout with 16-byte alignment
pub struct Color {
    pub r: f32,  // Red component [0.0, 1.0]
    pub g: f32,  // Green component [0.0, 1.0] 
    pub b: f32,  // Blue component [0.0, 1.0]
    pub a: f32,  // Alpha component [0.0, 1.0]
}
```

**Required Methods:**
- `new(r: f32, g: f32, b: f32, a: f32) -> Self` (const fn where stable, clamp all inputs to [0.0, 1.0])
- `rgb(r: f32, g: f32, b: f32) -> Self` (const fn where stable, alpha = 1.0, clamp inputs)
- `rgba(r: u8, g: u8, b: u8, a: u8) -> Self` (convert from 0-255 values)
- `from_hex(hex: u32) -> Self` (from 0xRRGGBB format, alpha = 1.0, mask to 24 bits)
- `from_hex_alpha(hex: u32) -> Self` (from 0xRRGGBBAA format, use all 32 bits)
- `from_hex_str(hex: &str) -> Result<Self, ColorParseError>` (parse "#RRGGBB" or "#RRGGBBAA" format)
- `to_rgba_u8(&self) -> (u8, u8, u8, u8)` (convert to 0-255 values, round to nearest)
- `with_alpha(&self, alpha: f32) -> Self` (create new color with clamped alpha)
- `is_valid(&self) -> bool` (check if all components are in [0.0, 1.0] range)

**Error Types:**
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum ColorParseError {
    InvalidFormat,     // Not in #RRGGBB or #RRGGBBAA format
    InvalidCharacter,  // Non-hex character found
    InvalidLength,     // Wrong string length
}
```

**Helper Functions (Internal):**
- `clamp01(value: f32) -> f32` (saturating clamp to [0.0, 1.0], used by all constructors)

**Required Constants (Exact Values):**
- `WHITE: Color = Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }`
- `BLACK: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }`
- `RED: Color = Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }`
- `GREEN: Color = Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 }`
- `BLUE: Color = Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 }`
- `YELLOW: Color = Color { r: 1.0, g: 1.0, b: 0.0, a: 1.0 }`
- `CYAN: Color = Color { r: 0.0, g: 1.0, b: 1.0, a: 1.0 }`
- `MAGENTA: Color = Color { r: 1.0, g: 0.0, b: 1.0, a: 1.0 }`
- `TRANSPARENT: Color = Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }`

**Validation & Error Handling:**
- **Clamping Strategy**: Saturating clamp (values < 0.0 → 0.0, values > 1.0 → 1.0)
- **When to Clamp**: At construction time in all methods
- **Invalid Hex Handling**: Mask input values (0xRRGGBB & 0xFF_FF_FF, 0xRRGGBBAA & 0xFF_FF_FF_FF)
- **Precision**: Use f32::round() for u8 conversions to ensure proper rounding

### File: `src/style.rs`
Implement basic styling types:

#### 2. Padding Struct
```rust
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]  // Memory layout consistency
pub struct Padding {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}
```

**Required Methods:**
- `new(left: f32, right: f32, top: f32, bottom: f32) -> Self` (const fn where stable, clamp negative to 0.0)
- `all(value: f32) -> Self` (const fn where stable, clamp negative to 0.0)
- `horizontal(value: f32) -> Self` (const fn where stable, left=right=value, top=bottom=0.0)
- `vertical(value: f32) -> Self` (const fn where stable, top=bottom=value, left=right=0.0)
- `symmetric(horizontal: f32, vertical: f32) -> Self` (const fn where stable, ergonomic constructor)
- `zero() -> Self` (const fn where stable, all values = 0.0)

**Validation:**
- **Negative values**: Clamp to 0.0 (padding cannot be negative)
- **Maximum values**: No upper limit (allows flexible layouts)

#### 3. Border Struct  
```rust
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]  // Memory layout consistency
pub struct Border {
    pub width: f32,
    pub color: Color,
}
```

**Future Roadmap Note**: This minimal Border implementation focuses on solid borders. Future versions will support border radius, different border styles (dashed, dotted), and per-side border widths.

**Required Methods:**
- `new(width: f32, color: Color) -> Self` (const fn where stable, clamp width to >= 0.0)
- `none() -> Self` (const fn where stable, width=0.0, color=TRANSPARENT)
- `solid(width: f32, color: Color) -> Self` (const fn where stable, same as new but clearer intent)

**Validation:**
- **Border width**: Clamp negative values to 0.0
- **Color dependency**: Use clamped Color values

## Implementation Standards
- Follow Rust 2024 edition conventions
- Use `#![warn(missing_docs)]` compliance
- Implement `const fn` where possible for performance (limited by stable Rust capabilities)
- Use `#[repr(C)]` for GPU-compatible memory layout
- Add comprehensive unit tests in same file with `approx` crate for float comparisons
- All color values use f32 for GPU compatibility
- Handle edge cases with clear validation (clamp values, invalid inputs)
- Provide conversion utilities between different color formats
- Use saturating clamp for all validation (never panic on invalid input)

## Acceptance Criteria
- [ ] All types compile without warnings with `#[repr(C)]`
- [ ] Complete rustdoc documentation for all public APIs with examples
- [ ] Unit tests cover normal, edge cases, and conversion functions
- [ ] Property-based tests for color conversion accuracy (rgba ↔ u8 ↔ hex)
- [ ] Benchmark tests for color conversion performance
- [ ] `cargo test` passes for color and style modules
- [ ] Performance: operations are zero-cost abstractions with `const fn`
- [ ] Color validation and clamping works correctly in all scenarios
- [ ] Hex color parsing handles all invalid inputs gracefully
- [ ] Memory layout verified for GPU compatibility (16-byte aligned)

## Performance Requirements
- **Color conversions**: Microbenchmarks should target < 10ns per operation on modern x86_64 CPUs
- **Memory alignment**: 16-byte aligned for SIMD operations and GPU buffer compatibility  
- **GPU buffer layout**: Must match `float4` type in Metal/HLSL shaders with proper padding
- **Const evaluation**: All constants computed at compile time where Rust stable permits
- **Zero heap allocation**: All operations stack-only

## Design Decisions Rationale
- **Color Format**: RGBA with f32 components for GPU compatibility and precision
- **Value Range**: [0.0, 1.0] for all color components (OpenGL/DirectX standard)
- **Memory Layout**: `#[repr(C, align(16))]` ensures predictable layout for GPU buffers with SIMD alignment
- **Validation**: Automatic saturating clamp (never panic, always valid state) using internal `clamp01()` helper
- **Hex Format**: Support both RGB (0xRRGGBB) and RGBA (0xRRGGBBAA) with proper masking
- **String Parsing**: CSS-style hex parsing for developer convenience (`from_hex_str`)
- **Constants**: Provide commonly used colors as const values with exact specifications
- **Dependency Direction**: Color → Style (Border depends on Color, not vice versa)

## Module Import Order
```rust
// src/lib.rs
pub mod color;    // No dependencies
pub mod style;    // Depends on color
```

## Test Coverage Requirements
### Unit Tests:
- Constructor methods (new, rgb, rgba, from_hex, from_hex_str)
- Conversion methods (to_rgba_u8, with_alpha)
- Edge cases (negative values, > 1.0 values, invalid hex, malformed strings)
- Constant values verification
- Helper function behavior (clamp01)

### Property-Based Tests:
- Round-trip conversions: `Color → u8 → Color` with tolerance ≤ 1/255 per component
- Hex parsing: `0xRRGGBB → Color → to_rgba_u8` accuracy within rounding error
- Clamping behavior: All inputs should produce valid [0.0, 1.0] output
- String parsing: CSS-style hex strings should parse correctly

### Benchmark Tests:
- Color conversion performance: target < 10ns per operation on x86_64
- Memory layout verification: confirm 16-byte alignment and GPU compatibility
- Batch operations: measure SIMD potential with aligned data

## Code Review Focus Areas
- Color conversion accuracy and performance (< 10ns requirement)
- Input validation and error handling (no panics, all clamps)
- API ergonomics for common use cases (builder patterns, constants)
- Memory layout efficiency for GPU usage (16-byte alignment)
- Documentation completeness with usage examples
- Cross-platform compatibility (endianness, float precision)

## Dependencies
- None (standalone module)
- Dev dependencies: approx crate (for float testing)

## Next Task
Task 1.2.3 - Implement Widget trait and basic widgets

## Notes
These color and styling types will be used extensively throughout the GUI framework for theming, drawing, and visual presentation. Focus on providing a clean, efficient API that supports both programmatic color creation and common color constants.