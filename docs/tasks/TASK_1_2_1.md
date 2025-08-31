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
pub struct Point {
    pub x: f32,
    pub y: f32,
}
```
**Required Methods:**
- `new(x: f32, y: f32) -> Self`
- `zero() -> Self` (const fn)
- Basic arithmetic operations (+, -, *, /)
- Distance calculations

**Derive:** `Debug, Clone, Copy, PartialEq`

#### 2. Size Struct  
```rust
pub struct Size {
    pub width: f32,
    pub height: f32,
}
```
**Required Methods:**
- `new(width: f32, height: f32) -> Self`
- `zero() -> Self` (const fn)
- Validation methods (is_valid, is_positive)
- Area calculation

**Derive:** `Debug, Clone, Copy, PartialEq`

#### 3. Rect Struct
```rust
pub struct Rect {
    pub pos: Point,
    pub size: Size,
}
```
**Required Methods:**
- `new(x: f32, y: f32, width: f32, height: f32) -> Self`
- `zero() -> Self` (const fn)
- `left()`, `right()`, `top()`, `bottom()` edge accessors
- `contains_point(point: Point) -> bool`
- `intersects(other: Rect) -> bool`
- `area() -> f32`

#### 4. Vec2 Type Alias
```rust
pub type Vec2 = Point;
```
For mathematical vector operations - add vector arithmetic implementations.

## Implementation Standards
- Follow Rust 2024 edition conventions
- Use `#![warn(missing_docs)]` compliance
- Implement `const fn` where possible for performance
- Add comprehensive unit tests in same file
- All coordinates use f32 for GPU compatibility
- Handle edge cases (negative sizes, zero areas, etc.)

## Acceptance Criteria
- [ ] All types compile without warnings
- [ ] Complete rustdoc documentation for all public APIs
- [ ] Unit tests cover normal and edge cases
- [ ] `cargo test` passes for math module
- [ ] Performance: operations should be zero-cost abstractions
- [ ] Code follows project style guidelines

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