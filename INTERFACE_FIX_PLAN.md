# Interface Range Loop Fix - Implementation Plan

> **STATUS (2026-03-05): COMPLETED** — The "Better Fix" approach below was implemented via VarTable
> in `go/vartable.go`. Interface parameters use `&dyn Trait`, method calls work directly on bare
> receivers, call sites use `r.borrow().as_ref().unwrap()` for coercion, and range loop iteration
> with `shape.as_ref()` works end-to-end. Tests: `interfaces_simple`, `interfaces_basic`,
> `interface_basic` all pass. See `go/vartable.go` for the implementation.

## The Problem

When iterating over a slice of interfaces in Go, the transpiled Rust code fails because:

1. The iterator yields `&Box<dyn Trait>` (a reference)
2. The function expects `Rc<RefCell<Option<Box<dyn Trait>>>>` (owned value)
3. Trait objects (`dyn Trait`) cannot be cloned

## Current Broken Code

### Go Input

```go
shapes := []Shape{rect, circle}
for i, shape := range shapes {
    printShapeInfo(shape)
}
```

### Rust Output (Broken)

```rust
let shapes = vec![Box::new(rect) as Box<dyn Shape>, Box::new(circle) as Box<dyn Shape>];
for (i, shape) in shapes.iter().enumerate() {
    // shape is &Box<dyn Shape>
    print_shape_info(Rc::new(RefCell::new(Some(shape.clone())))) // ERROR: cannot clone
}
```

## Quick Fix Implementation

### Step 1: Detect Interface Slice Iteration

In `stmt.go` around line 1232, enhance the detection:

```go
// Current code detects empty interface{}
if intf, ok := elemType.Underlying().(*types.Interface); ok {
    if intf.NumMethods() == 0 {
        valueType = "&Box<dyn Any>"
    } else {
        // NEW: Handle named interfaces
        if namedType, ok := elemType.(*types.Named); ok {
            valueType = "&Box<dyn " + namedType.Obj().Name() + ">"
            rangeLoopVars[valueName] = valueType
            isInterfaceSlice = true // Add flag
        }
    }
}
```

### Step 2: Special Case Interface Parameters

In `expr.go` around line 1876, handle interface references:

```go
if strings.HasPrefix(varType, "&Box<dyn ") {
    // It's a reference to a trait object
    if needsInterfaceBoxing {
        // Create a new wrapped reference
        // Instead of cloning (impossible), create a reference wrapper
        out.WriteString("{\n")
        out.WriteString("        let trait_ref = ")
        out.WriteString(ident.Name)
        out.WriteString(";\n")
        out.WriteString("        // Create wrapped reference\n")
        out.WriteString("        Rc::new(RefCell::new(Some(trait_ref.clone())))\n")
        out.WriteString("    }")
    }
}
```

## Better Fix: Change Interface Parameter Types

### Step 1: Modify Function Signature Generation

In `decl.go`, detect interface parameters:

```go
// When generating parameter type
paramType := field.Type
if IsInterfaceType(paramType) {
    // Don't wrap interface parameters
    out.WriteString("&dyn ")
    out.WriteString(GetInterfaceName(paramType))
} else {
    // Regular wrapping for other types
    out.WriteString(GoTypeToRust(paramType))
}
```

### Step 2: Update Function Calls

When calling functions with interface parameters:

```go
if paramExpectsInterface(funcSig, i) {
    // Pass as reference without wrapping
    out.WriteString(ident.Name)
    out.WriteString(".as_ref()")
} else {
    // Regular wrapped passing
    WriteWrappedValue(out, arg)
}
```

### Step 3: Update Method Calls on Interfaces

Interface method calls need adjustment:

```go
// Instead of: (*s.borrow_mut().as_mut().unwrap()).method()
// Generate: s.method()
```

## Optimal Fix: Selective Wrapping

### Phase 1: Add Type Context

Create a context system to track how types are used:

```go
type TypeContext struct {
    IsParameter      bool
    IsReturnValue    bool
    IsStructField    bool
    IsLocalVariable  bool
    AddressTaken     bool
    CanBeNil        bool
}

func shouldWrapType(typ ast.Expr, ctx TypeContext) bool {
    // Interfaces in parameters are never wrapped
    if IsInterfaceType(typ) && ctx.IsParameter {
        return false
    }
    // ... other rules
}
```

### Phase 2: Modify GoTypeToRust

Make wrapping context-aware:

```go
func GoTypeToRustContext(expr ast.Expr, ctx TypeContext) string {
    baseType := goTypeToRustBase(expr)
    
    if !shouldWrapType(expr, ctx) {
        return baseType
    }
    
    // Apply wrapping only when needed
    return wrapType(baseType)
}
```

### Phase 3: Update All Type Generation

Propagate context throughout:

- Function parameters
- Return values  
- Struct fields
- Local variables

## Testing Strategy

1. **Start with interfaces_basic test**
   - Implement quick fix
   - Verify range loops work
   - Check method calls still work

2. **Verify no regressions**
   - Run full test suite
   - Ensure interfaces_simple still passes
   - Check interface_basic behavior

3. **Add new test cases**
   - Interface slices with nil values
   - Multiple interface types
   - Nested interfaces

## Implementation Order

1. **Immediate**: Quick fix for range loops (unblock tests)
2. **Next sprint**: Better fix with parameter type changes
3. **Future**: Full selective wrapping implementation

## Expected Outcome

After the quick fix:

```rust
// Range loop should work
for (i, shape) in shapes.iter().enumerate() {
    // shape is &Box<dyn Shape>
    print_shape_info(shape) // Pass reference directly
}
```

After the better fix:

```rust
// Cleaner function signature
fn print_shape_info(s: &dyn Shape) {
    println!("Area: {}", s.area());
}

// Natural iteration
for shape in &shapes {
    print_shape_info(shape.as_ref())
}
```

After optimal fix:

```rust
// Idiomatic Rust
fn print_shape_info(shape: &dyn Shape) -> f64 {
    shape.area()
}

for shape in &shapes {
    let area = print_shape_info(shape);
    println!("Area: {}", area);
}
```
