# Task 1.2.3: Design Widget Trait and Response Types

**Priority**: HIGH  
**Status**: ASSIGNED  
**Developer**: Team Member  
**Phase**: 1.2 Core Abstractions  
**Estimated Time**: 3-4 hours  

## Objective
Create the foundational widget trait and response system for immediate-mode GUI widget interactions and state management.

## Technical Requirements

### File: `src/widget.rs`
Implement the following traits and types with complete functionality:

#### 1. Widget Trait
```rust
pub trait Widget {
    type Response;
    
    /// Show the widget and return its response
    /// 
    /// This is the core immediate-mode GUI method - called every frame
    /// to both render the widget and handle interactions.
    fn show(self, ctx: &mut Context) -> Self::Response;
}
```

**Design Philosophy:**
- **Immediate-mode**: Widgets are stateless and called every frame
- **Consume self**: Widgets are consumed on use (Dear ImGui pattern)
- **Generic Response**: Each widget can define its own response type
- **Context dependency**: All widgets need rendering/input context

#### 2. Response System
```rust
#[derive(Debug, Clone)]
pub struct Response {
    /// The rectangular area this widget occupies
    pub rect: Rect,
    
    /// Whether the widget was clicked this frame
    pub clicked: bool,
    
    /// Whether the widget is currently being hovered
    pub hovered: bool,
    
    /// Whether the widget has keyboard focus
    pub focused: bool,
    
    /// Whether the widget was right-clicked this frame
    pub secondary_clicked: bool,
    
    /// Whether the widget was double-clicked this frame
    pub double_clicked: bool,
    
    /// Whether the widget received a key press this frame
    pub key_pressed: Option<Key>,
    
    /// Whether the widget's state changed this frame
    pub changed: bool,
}
```

**Required Methods:**
- `new(rect: Rect) -> Self` - Create basic response with only rect
- `with_click(mut self) -> Self` - Builder method to mark as clicked
- `with_hover(mut self) -> Self` - Builder method to mark as hovered
- `with_focus(mut self) -> Self` - Builder method to mark as focused
- `with_key_press(mut self, key: Key) -> Self` - Builder method for key input
- `union(self, other: Self) -> Self` - Combine responses (for composite widgets)

#### 3. Context System
```rust
pub struct Context {
    /// Current mouse position in screen coordinates
    pub mouse_pos: Point,
    
    /// Whether left mouse button was pressed this frame
    pub mouse_pressed: bool,
    
    /// Whether left mouse button was released this frame
    pub mouse_released: bool,
    
    /// Whether right mouse button was pressed this frame
    pub mouse_secondary_pressed: bool,
    
    /// Mouse wheel delta this frame
    pub mouse_wheel_delta: f32,
    
    /// Key that was pressed this frame (if any)
    pub key_pressed: Option<Key>,
    
    /// Currently focused widget ID (if any)
    pub focused_id: Option<WidgetId>,
    
    /// Hot widget ID (mouse hover target)
    pub hot_id: Option<WidgetId>,
    
    /// Active widget ID (mouse interaction target)
    pub active_id: Option<WidgetId>,
    
    /// Current frame counter for state management
    pub frame_count: u64,
    
    /// Current theme settings
    pub theme: Theme,
    
    /// Current text input buffer
    pub text_input: String,
}
```

**Required Methods:**
- `new() -> Self` - Create default context
- `is_widget_active(&self, id: WidgetId) -> bool` - Check if widget is active
- `is_widget_hot(&self, id: WidgetId) -> bool` - Check if widget is hot
- `is_widget_focused(&self, id: WidgetId) -> bool` - Check if widget is focused
- `set_active(&mut self, id: Option<WidgetId>)` - Set active widget
- `set_hot(&mut self, id: Option<WidgetId>)` - Set hot widget
- `set_focused(&mut self, id: Option<WidgetId>)` - Set focused widget
- `request_focus(&mut self, id: WidgetId)` - Request focus for widget
- `contains_mouse(&self, rect: Rect) -> bool` - Check if mouse is in rect

#### 4. Widget ID System
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WidgetId(u64);

impl WidgetId {
    /// Create a new unique widget ID
    pub fn new() -> Self;
    
    /// Create a widget ID from a string (for stable IDs)
    pub fn from_str(s: &str) -> Self;
    
    /// Create a widget ID from a hash of values
    pub fn from_hash(hash: u64) -> Self;
}
```

#### 5. Input Types
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    // Printable characters
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,
    
    // Special keys
    Space,
    Enter,
    Escape,
    Backspace,
    Delete,
    Tab,
    
    // Arrow keys
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    
    // Modifier keys
    Shift,
    Control,
    Alt,
    Command,
}
```

#### 6. Theme System
```rust
#[derive(Debug, Clone)]
pub struct Theme {
    /// Primary brand color
    pub primary: Color,
    
    /// Secondary brand color
    pub secondary: Color,
    
    /// Background color for panels/windows
    pub background: Color,
    
    /// Text color
    pub text: Color,
    
    /// Border color
    pub border: Color,
    
    /// Hover state color
    pub hover: Color,
    
    /// Active/pressed state color
    pub active: Color,
    
    /// Focus indicator color
    pub focus: Color,
    
    /// Default padding for widgets
    pub default_padding: Padding,
    
    /// Default border for widgets
    pub default_border: Border,
}
```

**Required Methods:**
- `default() -> Self` - Create default light theme
- `dark() -> Self` - Create default dark theme

### File: `src/widgets/mod.rs`
Create widget module structure:

```rust
//! Widget implementations for Ho GUI

pub mod button;
pub mod text;
pub mod container;

// Re-export common widgets
pub use button::Button;
pub use text::Text;
pub use container::Container;
```

#### 7. Example Widget Implementation - Button
```rust
// File: src/widgets/button.rs

pub struct Button<'a> {
    text: &'a str,
    id: WidgetId,
    padding: Option<Padding>,
    color: Option<Color>,
}

impl<'a> Button<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            id: WidgetId::from_str(text), // Simple ID strategy
            padding: None,
            color: None,
        }
    }
    
    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = Some(padding);
        self
    }
    
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
    
    pub fn id(mut self, id: WidgetId) -> Self {
        self.id = id;
        self
    }
}

impl<'a> Widget for Button<'a> {
    type Response = Response;
    
    fn show(self, ctx: &mut Context) -> Self::Response {
        let padding = self.padding.unwrap_or(ctx.theme.default_padding);
        let color = self.color.unwrap_or(ctx.theme.primary);
        
        // Calculate widget size (simplified)
        let text_size = measure_text(self.text, &ctx.theme);
        let widget_size = Size::new(
            text_size.width + padding.left + padding.right,
            text_size.height + padding.top + padding.bottom,
        );
        
        // Get widget rect (simplified - needs proper layout)
        let rect = Rect::new(0.0, 0.0, widget_size.width, widget_size.height);
        
        // Handle interactions
        let is_hovered = ctx.contains_mouse(rect);
        let is_active = ctx.is_widget_active(self.id);
        let clicked = is_hovered && ctx.mouse_released && is_active;
        
        // Update context state
        if is_hovered && ctx.mouse_pressed {
            ctx.set_active(Some(self.id));
        }
        if !ctx.mouse_pressed {
            ctx.set_active(None);
        }
        if is_hovered {
            ctx.set_hot(Some(self.id));
        }
        
        // TODO: Add actual rendering commands here
        
        // Build response
        Response::new(rect)
            .with_hover_if(is_hovered)
            .with_click_if(clicked)
    }
}

// Helper function for text measurement (placeholder)
fn measure_text(text: &str, theme: &Theme) -> Size {
    // Simplified text measurement
    Size::new(text.len() as f32 * 8.0, 16.0)
}
```

## Implementation Standards
- Follow Rust 2024 edition conventions
- Use `#![warn(missing_docs)]` compliance
- Implement immediate-mode patterns (stateless widgets)
- Add comprehensive unit tests for interaction logic
- Use builder patterns for widget configuration
- Handle all input edge cases gracefully
- Provide clear documentation with usage examples
- Focus on zero-allocation per-frame operations

## Acceptance Criteria
- [ ] Widget trait compiles and supports generic responses
- [ ] Response system handles all common interaction types
- [ ] Context system tracks input state correctly
- [ ] Widget ID system provides stable, unique identifiers
- [ ] Theme system supports common styling needs
- [ ] Button example widget demonstrates complete interaction cycle
- [ ] Unit tests cover interaction logic and edge cases
- [ ] Documentation includes usage examples and patterns
- [ ] `cargo test` passes for widget module
- [ ] Performance: zero heap allocation during normal operation

## Performance Requirements
- **Widget creation**: Target < 50ns per widget instantiation
- **Response handling**: Zero allocation for common interactions
- **Context updates**: Minimal state tracking overhead
- **ID generation**: Fast hash-based ID generation
- **Theme access**: Const-time theme property access

## Design Decisions Rationale
- **Immediate-mode**: Widgets consumed on use for cleaner API and less state management
- **Generic Response**: Allows different widgets to return specialized response types
- **Builder Pattern**: Fluent API for widget configuration
- **Context Centralization**: Single source of truth for input/theme state
- **Widget ID**: Stable IDs for focus/hover tracking across frames
- **Response Union**: Allows composite widgets to combine child responses
- **Theme Integration**: Built-in theming support from the foundation

## Module Dependencies
```
src/widget.rs
├── math (Point, Size, Rect)
├── color (Color)
├── style (Padding, Border)
└── widgets/
    ├── button.rs
    ├── text.rs
    └── container.rs
```

## Test Coverage Requirements
### Unit Tests:
- Widget trait basic functionality
- Response builder methods and combinations
- Context state management (active, hot, focused)
- Widget ID generation and stability
- Theme property access
- Button widget interaction logic

### Integration Tests:
- Complete widget interaction cycles
- Multi-widget focus management
- Event propagation and handling
- Theme switching behavior

### Example Tests:
- Simple button click detection
- Hover state transitions
- Keyboard focus navigation
- Multi-frame interaction sequences

## Code Review Focus Areas
- Immediate-mode API ergonomics and safety
- Input state management correctness
- Widget ID collision prevention
- Response type flexibility and extensibility
- Theme system completeness and consistency
- Memory allocation patterns (should be zero per frame)
- Documentation clarity for immediate-mode concepts

## Dependencies
- `math` module (Point, Size, Rect types)
- `color` module (Color type)
- `style` module (Padding, Border types)

## Next Task
Task 1.2.4 - Implement DrawContext and basic drawing primitives

## Notes
This widget system establishes the foundation for immediate-mode GUI interactions. Focus on creating a clean, ergonomic API that feels natural for immediate-mode programming while maintaining the performance characteristics needed for 60fps GUIs. The Response system should be flexible enough to support both simple and complex widget interactions.