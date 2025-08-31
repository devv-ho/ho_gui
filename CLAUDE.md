# Ho GUI Framework

## Project Overview
An immediate-mode GUI library for Rust, designed for building general desktop applications with a clean, imgui-inspired API.

## Project Context
- **Architecture**: Immediate-mode GUI (call widgets each frame)
- **Primary Target**: macOS (Metal/Core Graphics backend)
- **Future Support**: Windows (DirectX/GDI backend)
- **Rendering**: CPU fallback + GPU acceleration when available
- **API Style**: Immediate-mode like Dear ImGui but with Rust safety

## Project Structure
```
ho_gui/
├── src/
│   ├── lib.rs                 // Public API exports
│   ├── app.rs                 // Application context & event loop
│   ├── math.rs                // Point, Size, Rect, Vec2 types
│   ├── color.rs               // Color types and utilities
│   ├── widgets/               // Widget implementations
│   ├── layout/                // Layout system
│   ├── rendering/             // Rendering pipeline & backends
│   ├── input/                 // Event handling system
│   └── style/                 // Styling and theming
├── examples/                  // Example applications
└── tests/                     // Integration tests
```

## Development Commands
- **Build**: `cargo build`
- **Test**: `cargo test`
- **Check**: `cargo check`
- **Examples**: `cargo run --example hello_world`
- **Docs**: `cargo doc --open`
- **Format**: `cargo fmt`
- **Lint**: `cargo clippy`

## Code Standards
- Follow Rust 2024 edition conventions
- Use `#![warn(missing_docs)]` for public APIs
- Prefer composition over inheritance
- Zero-cost abstractions where possible
- Memory-safe immediate-mode patterns
- Comprehensive error handling with custom error types

## API Design Principles
- **Immediate Mode**: Widgets called each frame, no retained state
- **Builder Pattern**: Fluent, chainable widget configuration
- **Type Safety**: Leverage Rust's type system for correctness
- **Performance**: 60fps target for typical desktop applications
- **Simplicity**: Minimal boilerplate for common use cases

## Dependencies Strategy
- **Core**: `winit` (windowing), `wgpu` (GPU), `tiny-skia` (CPU rendering)
- **Text**: `fontdb` (fonts), `rustybuzz` (text shaping)
- **Math**: `glam` (linear algebra)
- **Minimal**: Prefer std library when possible

## Architecture Notes
- Widget trait system for extensibility
- Command-pattern rendering pipeline
- Event propagation with response system
- Modular backend system (Metal/Software/Future DirectX)
- Frame-based immediate-mode with retained optimizations

## Performance Goals
- 60fps for typical desktop applications
- Minimal allocations per frame
- Efficient text rendering and layout
- GPU acceleration with CPU fallback
- Memory usage under 50MB for typical apps

## Testing Strategy
- Unit tests for core components
- Integration tests for widget behavior
- Visual regression tests for rendering
- Performance benchmarks
- Example applications as integration tests

## Release Strategy
- Phase 1: Core foundation (math, rendering, basic widgets)
- Phase 2: Advanced widgets and layout system
- Phase 3: GPU acceleration and performance optimization
- Phase 4: Polish, documentation, and 0.1.0 release

## Current Development Phase
**Phase 1 - Foundation**: Setting up project structure, core types, and basic rendering pipeline.

## Code Review Focus
- Memory safety in immediate-mode patterns
- API ergonomics and usability
- Performance implications of design decisions
- Cross-platform compatibility considerations
- Documentation completeness