# Ho GUI Framework - Architecture Documentation

## 📖 How to Use This Documentation

### For Claude/AI: Quick Navigation Guide
This documentation is split across multiple files to ensure readability. When asked to understand the project:

1. **Start here** - `PROJECT_ARCHITECTURE.md` (this file) - Overview and navigation
2. **Core modules** - `docs/MATH_MODULE.md` - Point, Size, Rect, Vec2 implementation
3. **Color system** - `docs/COLOR_MODULE.md` - RGBA color with GPU optimization
4. **Quick reference** - Use section headers to jump to specific topics

### Information Lookup Strategy
- **Architecture questions** → This file (overview, principles, dependencies)
- **Math operations** → `docs/MATH_MODULE.md` (Point, Rect, geometric operations)
- **Color handling** → `docs/COLOR_MODULE.md` (RGBA, hex parsing, GPU layout)
- **Performance concerns** → Search for "Performance" in relevant module docs
- **Testing patterns** → Search for "Testing" in module-specific files

### Document Structure Pattern
Each module doc follows this pattern:
```
1. Core Design Philosophy
2. Type Definitions & Memory Layout
3. Key Methods & Usage Patterns
4. Critical Implementation Details
5. Performance Characteristics
6. Testing Strategy
```

### Adding New Documentation
When extending docs:
1. Follow the same structure pattern above
2. Update this navigation guide with new file references
3. Add cross-references between related concepts
4. Keep individual files under 1000 lines for AI readability

---

## Project Overview

Ho GUI is an immediate-mode GUI library for Rust, designed for building general desktop applications with a clean, imgui-inspired API. This document provides comprehensive technical documentation of the project structure and implementation details for future development reference.

### Key Architecture Principles
- **Immediate Mode**: Widgets called each frame, no retained state
- **Type Safety**: Leverage Rust's type system for correctness  
- **Performance**: 60fps target for typical desktop applications
- **GPU Compatibility**: Memory layouts optimized for GPU buffers
- **Zero-Cost Abstractions**: Const functions and compile-time evaluation where possible

### Primary Target Platforms
- **Primary**: macOS (Metal/Core Graphics backend)
- **Future**: Windows (DirectX/GDI backend)
- **Rendering**: CPU fallback + GPU acceleration when available

## Detailed Module Documentation

**Complete technical details are available in separate files:**

### Math Module → [`docs/MATH_MODULE.md`](docs/MATH_MODULE.md)
- Point, Size, Rect, Vec2 types and operations
- Arithmetic operations and safety considerations  
- Hit testing and geometric algorithms
- Performance characteristics and testing strategies

### Color Module → [`docs/COLOR_MODULE.md`](docs/COLOR_MODULE.md)
- RGBA color system with GPU optimization
- Hex parsing and CSS-style string support
- Saturating clamp strategy and NaN handling
- Const function implementation and performance benchmarks

---

## Module Dependencies

```
src/lib.rs
├── color     (no dependencies)
└── math      (no dependencies) 
```

Both modules are standalone with no external dependencies except for testing (`approx` crate for float comparisons).

---

## Development Guidelines

### Performance Requirements
- **Color operations**: Target <10ns per operation on modern x86_64
- **Memory alignment**: 16-byte aligned for GPU compatibility
- **Zero allocation**: All operations must be stack-only

### Code Standards
- Use `#![warn(missing_docs)]` for all public APIs
- Const functions where stable Rust permits
- Comprehensive error handling with custom error types
- Property-based testing for critical algorithms

### Testing Strategy
- Unit tests for all public methods
- Edge case testing (NaN, infinity, boundary conditions)
- Property-based tests for round-trip conversions
- Performance regression tests
- Memory layout verification

### API Design Principles
- **Immutable by default**: Constructors return new instances
- **Builder pattern**: Fluent, chainable configuration
- **Type safety**: Leverage Rust's type system
- **Clear semantics**: Method names indicate mutation vs creation

---

## Future Expansion Points

### Phase 2 Additions
- **Style Module**: Building on Color for comprehensive theming
- **Layout System**: Using math types for flexible positioning
- **Widget System**: Immediate-mode API leveraging both modules

### Optimization Opportunities
- SIMD acceleration for color operations
- GPU compute shader integration
- Advanced color spaces (HSV, LAB)
- Vectorized batch operations

### Cross-Platform Considerations
- Endianness handling for color byte order
- Platform-specific GPU alignment requirements
- Performance characteristics across architectures

---

This documentation serves as the definitive reference for the Ho GUI framework's foundation modules, ensuring consistent development practices and architectural understanding as the project scales.