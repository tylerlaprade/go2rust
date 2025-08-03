# go/types Integration

This document describes the integration of go/types for replacing heuristics in the go2rust transpiler.

## Overview

The transpiler now uses Go's type checker (`go/types`) to get accurate type information instead of relying on variable name heuristics. This provides:

1. **Accurate type detection** - No more guessing based on variable names
2. **Proper error handling** - Clear errors when type information is required but unavailable
3. **Future extensibility** - Easy to add more type-based transpilation logic

## Implementation

### Type Information Flow

1. **Parse once, use everywhere**: AST files are parsed once and reused for both type checking and transpilation
2. **Global context**: Type information is stored in a global context accessible during transpilation
3. **Graceful degradation**: If type checking fails (e.g., missing imports), the transpiler continues with warnings

### Key Components

- `typeinfo.go`: Provides type checking and helper methods for common type queries
- `context.go`: Manages the global transpilation context
- Type-aware functions: Print formatting, index expressions, range loops, etc.

## Error Handling

When type information is not available, the transpiler now:

1. **Generates error comments** in the output code
2. **Uses `unimplemented!()` macros** for critical type decisions
3. **Provides clear error messages** indicating type information is required

Example:

```rust
/* ERROR: Cannot determine if map or slice access - type information required */
unimplemented!("type info required for index expression")
```

## Future Improvements

With go/types integrated, we can now:

1. **Remove all wrapping for non-addressable expressions**
2. **Optimize method calls** based on receiver types
3. **Handle interfaces properly** with type assertions
4. **Generate more idiomatic Rust** based on actual Go types
5. **Support generic types** (Go 1.18+)
