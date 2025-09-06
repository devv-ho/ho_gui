# Task 1.2.1: Implement Basic Math Types

**Priority**: HIGH  
**Status**: ASSIGNED  
**Developer**: Team Member  
**Phase**: 1.2 Core Abstractions  
**Estimated Time**: 2-3 hours  

## Objective
Create the foundational math types required for GUI layout and positioning system.

## Technical Requirements

### File: `src/math.rs`
Implement the following types with complete functionality:

#### 1. Point Struct
```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}
```
**Required Methods:**
- `new(x: f32, y: f32) -> Self` (const fn)
- `zero() -> Self` (const fn)
- `distance_to(&self, other: &Self) -> f32`
- **Arithmetic Operations:** Add, Sub, Mul, Div (with Assign variants)
- **Panic Handling:** Division by zero panics with clear error message

**Required Traits:** All operations implement proper error handling and documentation

#### 2. Size Struct  
```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}
```
**Required Methods:**
- `new(width: f32, height: f32) -> Self` (const fn)
- `zero() -> Self` (const fn)
- `is_valid(&self) -> bool` (checks width >= 0.0 && height >= 0.0)
- `is_positive(&self) -> bool` (checks width > 0.0 && height > 0.0)
- `area(&self) -> f32` (panics on invalid size with clear error message)

**Validation Logic:** is_valid() = is_positive() ∪ has_zero_dimensions

#### 3. Rect Struct
```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub pos: Point,
    pub size: Size,
}
```
**Required Methods:**
- `new(x: f32, y: f32, width: f32, height: f32) -> Self` (const fn)
- `zero() -> Self` (const fn)
- `left()`, `right()`, `top()`, `bottom()` -> f32 (edge coordinate accessors)
- `contains_point(&self, point: Point) -> bool` (inclusive boundaries)
- `intersects(&self, other: Self) -> bool` (inclusive boundaries, symmetric)
- `area(&self) -> f32` (delegates to size.area())

**Coordinate System:** Y-down screen coordinates (Y increases downward)
**Boundary Policy:** Inclusive boundaries (points on edges are contained/intersecting)

#### 4. Vec2 Type Alias
```rust
pub type Vec2 = Point;
```
For mathematical vector operations - add vector arithmetic implementations.

## Implementation Standards
- Follow Rust 2024 edition conventions
- Use `#![warn(missing_docs)]` compliance
- Implement `const fn` where possible for performance
- Add comprehensive unit tests in same file with `approx` crate for float comparisons
- All coordinates use f32 for GPU compatibility
- Handle edge cases with clear panic messages (division by zero, invalid sizes)
- Use inclusive boundary policies for GUI interactions

## Acceptance Criteria
- [x] All types compile without warnings
- [x] Complete rustdoc documentation for all public APIs with `# Panics` sections
- [x] Unit tests cover normal, edge, and panic cases using `assert_relative_eq!`
- [x] `cargo test` passes for math module
- [x] Performance: operations are zero-cost abstractions with `const fn`
- [x] Code follows project style guidelines with modern format strings

## Design Decisions Made
- **Coordinate System**: Y-down screen coordinates for GUI compatibility
- **Boundary Policy**: Inclusive boundaries (edges count as contained/intersecting)
- **Error Handling**: Panic on invalid operations (division by zero, negative area calculation)
- **Validation**: Separate validation methods rather than constructor enforcement
- **Float Testing**: Use `approx` crate with epsilon comparisons for all float assertions

## Code Review Focus Areas
- API ergonomics and mathematical correctness
- Memory layout efficiency
- Documentation completeness and clarity
- Test coverage adequacy
- Future GPU shader compatibility

## Dependencies
- None (foundational module)

## Next Task
Task 1.2.2 - Implement Color and Style types

## Notes
These types will be used extensively throughout the framework. Focus on clean, intuitive mathematical APIs that feel natural to GUI developers.