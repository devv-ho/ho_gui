# Math Module Documentation (`src/math.rs`)

## 1. Core Design Philosophy

The math module provides fundamental 2D geometric types with complete arithmetic operations and GPU-compatible memory layouts for immediate-mode GUI operations.

**Key Principles:**
- **Complete Arithmetic**: Full operator overloading for natural mathematical expressions
- **Safety First**: Explicit panics on invalid operations (division by zero) vs silent errors
- **GPU Compatibility**: f32 precision and memory layouts suitable for vertex buffers
- **Zero-Cost Abstractions**: Const functions and stack-only operations
- **UI Coordinate System**: Top-left origin, positive Y downward (standard UI convention)

## 2. Type Definitions & Memory Layout

### Point
```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}
```

**Purpose**: 2D coordinate representation for positioning
**Memory**: 8 bytes (2 × f32)
**Coordinate System**: Top-left origin (0,0), +X right, +Y down

### Size
```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}
```

**Purpose**: Rectangle dimensions with validation
**Memory**: 8 bytes (2 × f32)
**Constraints**: Must be non-negative for validity

### Rect
```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub pos: Point,
    pub size: Size,
}
```

**Purpose**: Positioned rectangle for layout and hit testing
**Memory**: 16 bytes (Point + Size)
**Layout**: top-left position + width/height

### Vec2
```rust
pub type Vec2 = Point;
```

**Purpose**: Type alias for 2D vectors/directions
**Rationale**: Same operations as Point but semantic clarity for movement/direction

## 3. Key Methods & Usage Patterns

### Point Operations
```rust
// Construction
let origin = Point::zero();
let pos = Point::new(10.0, 20.0);

// Distance calculation
let distance = point1.distance_to(&point2);

// Arithmetic (all operators supported)
let moved = pos + Vec2::new(5.0, 5.0);
let scaled = pos * Point::new(2.0, 2.0);
```

### Size Operations
```rust
// Construction and validation
let size = Size::new(100.0, 50.0);
assert!(size.is_valid());     // width >= 0.0 && height >= 0.0
assert!(size.is_positive());  // width > 0.0 && height > 0.0

// Safe area calculation
let area = size.area(); // Panics if invalid
```

### Rect Operations
```rust
// Construction
let rect = Rect::new(10.0, 20.0, 100.0, 50.0);

// Boundary queries
let left = rect.left();     // pos.x
let right = rect.right();   // pos.x + size.width
let top = rect.top();       // pos.y
let bottom = rect.bottom(); // pos.y + size.height

// Hit testing and collision
if rect.contains_point(mouse_pos) { /* ... */ }
if rect1.intersects(rect2) { /* ... */ }
```

## 4. Critical Implementation Details

### Division Safety Strategy
```rust
impl Div for Point {
    fn div(self, other: Self) -> Self::Output {
        if other.x == 0.0 || other.y == 0.0 {
            panic!("Attempted to divide {:?} by {:?}. (division-by-zero)", self, other);
        }
        // ... division logic
    }
}
```

**Design Decision**: Explicit panic vs NaN propagation
- **Rationale**: Clear error detection over silent corruption
- **Alternative**: Could return `Option<Point>` but breaks natural arithmetic flow
- **Usage**: Division by zero indicates logical error in GUI calculations

### Size Validation Philosophy
```rust
impl Size {
    pub fn is_valid(&self) -> bool {
        self.width >= 0.0 && self.height >= 0.0
    }
    
    pub fn area(&self) -> f32 {
        if !self.is_valid() {
            panic!("Attempted to get area of an invalid size. (invalid-argument)");
        }
        self.width * self.height
    }
}
```

**Design Decision**: Validation vs automatic clamping
- **Rationale**: Negative sizes indicate logical errors that should be caught
- **Alternative**: Auto-clamp to 0.0 (like Color module) - rejected for different semantics
- **Usage**: Invalid sizes should be prevented at creation, not silently corrected

### Intersection Algorithm
```rust
pub fn intersects(&self, other: Self) -> bool {
    self.left() <= other.right()
        && self.right() >= other.left()
        && self.top() <= other.bottom()
        && self.bottom() >= other.top()
}
```

**Implementation Notes:**
- Uses `<=` and `>=` for inclusive boundary testing
- Returns `true` for touching rectangles (edge contact)
- Handles nested rectangles correctly (smaller inside larger)

### Point Containment
```rust
pub fn contains_point(&self, point: Point) -> bool {
    point.x >= self.left()
        && point.x <= self.right()
        && point.y <= self.bottom()
        && point.y >= self.top()
}
```

**Boundary Behavior**: Inclusive on all edges
- **Design Decision**: Points exactly on rectangle edges are considered "inside"
- **Rationale**: Consistent with UI interaction expectations
- **Alternative**: Exclusive right/bottom edges - rejected for UI usability

## 5. Performance Characteristics

### Memory Efficiency
- **Stack-only operations**: No heap allocation for any math operations
- **Copy semantics**: All types are `Copy` for efficient parameter passing
- **Cache-friendly**: Small, aligned structures (8-16 bytes)

### Const Function Usage
```rust
pub const fn new(x: f32, y: f32) -> Self { Self { x, y } }
pub const fn zero() -> Self { Self::new(0.0, 0.0) }
```

**Benefits:**
- Compile-time evaluation for known values
- Zero runtime cost for constant initialization
- Enables const static declarations

### GPU Compatibility Notes
- **f32 precision**: Matches GPU shader requirements
- **Layout**: Compatible with vertex buffer uploads
- **Alignment**: Natural alignment suitable for GPU consumption

## 6. Testing Strategy

### Comprehensive Coverage Areas
```rust
mod point_tests {
    // Construction and basic operations
    test_create_new_point()
    test_create_negative_point()
    test_create_zero_point()
    
    // Arithmetic operations
    test_add_two_points()
    test_sub_two_points()
    test_mul_two_points()
    test_div_two_points()
    
    // Assignment operators
    test_add_assigning_other_point()
    test_div_assign_by_zero() // should_panic
    
    // Geometric operations
    test_calculate_distance_between_points()
}
```

### Edge Case Testing
- **Division by zero**: Explicit panic testing with `#[should_panic]`
- **Negative coordinates**: Ensuring proper handling of negative values
- **Floating-point precision**: Using `approx` crate with `TEST_EPSILON = 1e-6`
- **Boundary conditions**: Testing inclusive/exclusive edge behavior

### Property-Based Testing Opportunities
```rust
// Future additions could include:
// - Commutative property testing (a + b == b + a)
// - Associative property testing ((a + b) + c == a + (b + c))
// - Distance triangle inequality
// - Rectangle intersection symmetry
```

### Performance Testing
- **Const evaluation**: Verifying compile-time computation
- **Memory layout**: Ensuring expected sizes (Point: 8 bytes, Rect: 16 bytes)
- **No allocation**: Stack-only operation verification

## 7. Common Usage Patterns

### Layout Calculations
```rust
// Center a child rectangle within parent
fn center_rect(parent: Rect, child_size: Size) -> Rect {
    let center_x = parent.pos.x + (parent.size.width - child_size.width) / 2.0;
    let center_y = parent.pos.y + (parent.size.height - child_size.height) / 2.0;
    Rect::new(center_x, center_y, child_size.width, child_size.height)
}

// Clamp rectangle to bounds
fn clamp_rect(rect: Rect, bounds: Rect) -> Rect {
    let x = rect.pos.x.max(bounds.left()).min(bounds.right() - rect.size.width);
    let y = rect.pos.y.max(bounds.top()).min(bounds.bottom() - rect.size.height);
    Rect::new(x, y, rect.size.width, rect.size.height)
}
```

### Input Handling
```rust
// Mouse hit testing
fn handle_click(mouse_pos: Point, widgets: &[Widget]) -> Option<&Widget> {
    widgets.iter().find(|widget| widget.bounds.contains_point(mouse_pos))
}

// Collision detection
fn check_overlap(moving_rect: Rect, static_rects: &[Rect]) -> bool {
    static_rects.iter().any(|rect| moving_rect.intersects(*rect))
}
```

### Animation and Movement
```rust
// Linear interpolation
fn lerp_point(start: Point, end: Point, t: f32) -> Point {
    start + (end - start) * Point::new(t, t)
}

// Movement with velocity
fn update_position(pos: Point, velocity: Vec2, dt: f32) -> Point {
    pos + velocity * Vec2::new(dt, dt)
}
```

## 8. Future Enhancement Areas

### Potential Additions
- **Rotation operations**: Point rotation around origin/pivot
- **Matrix transformations**: 2D transformation matrix support
- **Bezier curves**: Quadratic/cubic curve support for advanced layouts
- **SIMD optimizations**: Vectorized operations for batch processing

### API Extensions
- **Builder patterns**: Fluent rectangle construction
- **Constraint solving**: Automatic layout constraint resolution
- **Advanced geometry**: Circle, ellipse, polygon support
- **Spatial queries**: Quadtree/spatial hashing integration

### Performance Optimizations
- **SIMD intrinsics**: AVX/SSE optimizations for bulk operations
- **GPU compute**: Offload complex calculations to GPU
- **Cache optimization**: Memory layout improvements for large datasets

---

**Cross-references:**
- Main architecture: `../PROJECT_ARCHITECTURE.md`
- Color system: `COLOR_MODULE.md`
- Implementation: `../src/math.rs`