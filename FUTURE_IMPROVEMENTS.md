# Future Improvements for Go2Rust

This document outlines complex improvements and optimizations that could be implemented in the future. These are not required for correctness but would improve performance, code quality, or feature completeness.

## 1. Statement-Level Closure Capture Handling

### Current Problem
Closures that capture variables need to clone Arc references, but these clones are currently generated inline within expressions, causing syntax errors when closures appear in return statements or complex expressions.

### Proposed Solution
Implement a two-pass approach:
1. **First pass**: Analyze the AST to find all closures and their captured variables
2. **Second pass**: Generate clone statements at the appropriate statement level

```go
// Go code
func makeCounter() func() int {
    count := 0
    return func() int {
        count++
        return count
    }
}
```

Should generate:
```rust
fn make_counter() -> Arc<Mutex<Option<Box<dyn Fn() -> Arc<Mutex<Option<i32>>> + Send + Sync>>>> {
    let mut count = Arc::new(Mutex::new(Some(0)));
    
    // Clone before the return statement, not inside it
    let count_captured = count.clone();
    
    return Arc::new(Mutex::new(Some(Box::new(move || {
        // Use count_captured here
        count_captured.lock().unwrap()...
    }))));
}
```

### Implementation Complexity
- Requires AST transformation before transpilation
- Need to track statement boundaries
- Handle nested closures and complex control flow

## 2. Escape Analysis for Rc vs Arc Optimization

### Current Approach
Everything uses `Arc<Mutex<Option<T>>>` for thread-safety and uniformity.

### Proposed Optimization
Use escape analysis to determine which variables never cross thread boundaries, then use the cheaper `Rc<RefCell<Option<T>>>` for those.

### Analysis Required
1. **Thread escape points**:
   - Variables used in `go` statements
   - Variables passed to functions that might spawn goroutines
   - Variables sent through channels
   - Variables stored in shared structures

2. **Safe for Rc**:
   - Local variables that never escape the function
   - Parameters not passed to goroutines
   - Temporary computation results
   - Loop variables in single-threaded contexts

### Implementation Challenges
- **Interprocedural analysis**: Need to track variables across function calls
- **Conservative assumptions**: When unsure, must use Arc for safety
- **Interface boundaries**: Need conversion between Rc and Arc at boundaries
- **Generic functions**: May need multiple versions or trait-based approach

### Potential Performance Gains
- Rc cloning is ~10x faster than Arc (no atomic operations)
- Reduced memory barriers
- Better cache locality

## 3. Lifetime Analysis for Reference Optimization

### Current Approach
All values are heap-allocated and reference-counted.

### Proposed Optimization
Use Rust's lifetime system where possible:
- Function parameters that aren't stored could be `&T` or `&mut T`
- Return values with clear ownership could avoid wrapping
- Stack allocation for non-escaping values

### Example Transformation
```go
func add(a, b int) int {
    return a + b
}
```

Currently generates:
```rust
fn add(a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> {
    // Complex unwrapping and rewrapping
}
```

Could generate:
```rust
fn add(a: &i32, b: &i32) -> i32 {
    a + b
}
```

### Requirements
- Full escape analysis
- Lifetime inference
- Handling of address-of operator edge cases
- Multiple function versions for different call sites

## 4. Proper Goroutine and Channel Support

### Current State
Goroutines and channels are not implemented.

### Proposed Implementation

#### Goroutines
```go
go func() {
    // work
}()
```

Should generate:
```rust
std::thread::spawn(move || {
    // work
});
```

#### Channels
Use `std::sync::mpsc` or `crossbeam-channel`:

```go
ch := make(chan int, 10)
ch <- 42
val := <-ch
```

Should generate:
```rust
let (tx, rx) = std::sync::mpsc::channel::<i32>();
tx.send(42).unwrap();
let val = rx.recv().unwrap();
```

### Challenges
- Select statements need special handling (maybe use `crossbeam-channel`)
- Buffered vs unbuffered semantics
- Closing channels
- Range over channels
- Bidirectional channels

## 5. Interface Method Resolution

### Current Limitation
Interface method calls aren't properly resolved.

### Proposed Solution
1. Generate trait definitions for each interface
2. Implement traits for concrete types
3. Use dynamic dispatch (`dyn Trait`) or static dispatch (generics)

```go
type Writer interface {
    Write([]byte) (int, error)
}
```

Should generate:
```rust
trait Writer {
    fn write(&mut self, data: &[u8]) -> Result<usize, Box<dyn Error>>;
}
```

### Challenges
- Interface embedding
- Type assertions
- Interface{} as Any
- Nil interface values

## 6. Generics Support

### Current State
Go generics (type parameters) are not supported.

### Proposed Implementation
Map Go generics to Rust generics:

```go
func Max[T constraints.Ordered](a, b T) T {
    if a > b {
        return a
    }
    return b
}
```

Should generate:
```rust
fn max<T: Ord>(a: T, b: T) -> T {
    if a > b { a } else { b }
}
```

### Challenges
- Constraint mapping
- Type inference
- Method sets on type parameters
- Type lists in constraints

## 7. Better Error Handling

### Current Approach
Errors are wrapped like any other value.

### Proposed Improvement
Use Rust's `Result<T, E>` type:

```go
func divide(a, b int) (int, error) {
    if b == 0 {
        return 0, errors.New("division by zero")
    }
    return a / b, nil
}
```

Should generate:
```rust
fn divide(a: i32, b: i32) -> Result<i32, Box<dyn Error>> {
    if b == 0 {
        return Err("division by zero".into());
    }
    Ok(a / b)
}
```

### Benefits
- More idiomatic Rust
- Better error propagation with `?`
- Type safety for error handling

## 8. Package and Module System

### Current Limitation
Single-file transpilation only.

### Proposed Solution
1. Analyze entire packages at once
2. Generate proper Rust module structure
3. Handle imports and exports correctly
4. Support package initialization

### Implementation
- Parse all `.go` files in a package together
- Generate `mod.rs` or `lib.rs` appropriately
- Map Go's package visibility to Rust's `pub`
- Handle circular dependencies (may require refactoring)

## 9. Standard Library Mapping

### Current Approach
Limited stdlib support with manual mappings.

### Proposed Improvement
Comprehensive mapping of Go's standard library to Rust equivalents:

| Go Package | Rust Equivalent |
|------------|----------------|
| `fmt` | `std::fmt`, `format!` macros |
| `io` | `std::io` |
| `os` | `std::env`, `std::fs`, `std::process` |
| `time` | `chrono` or `std::time` |
| `sync` | `std::sync`, `parking_lot` |
| `net/http` | `hyper`, `reqwest`, `actix-web` |
| `encoding/json` | `serde_json` |

### Challenges
- API differences
- Feature gaps
- Performance characteristics
- Async/await for I/O operations

## 10. Optimization Passes

### Proposed Optimizations

#### Dead Code Elimination
Remove unused variables and functions.

#### Constant Folding
Evaluate constant expressions at compile time.

#### Unnecessary Wrapper Removal
Detect when Arc<Mutex<Option<>>> isn't needed:
- Constants never need wrapping
- Immutable values might only need Arc
- Non-nil pointers might not need Option

#### Inline Simple Functions
Small functions could be inlined to reduce overhead.

### Implementation Approach
1. Build IR (intermediate representation)
2. Apply optimization passes
3. Generate optimized Rust code

## 11. Better Type Inference

### Current Limitation
Types are often explicitly specified even when Rust could infer them.

### Proposed Improvement
Let Rust's type inference do more work:

Currently generates:
```rust
let mut x: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(42)));
```

Could generate:
```rust
let mut x = Arc::new(Mutex::new(Some(42)));
```

## 12. Async/Await Support

### Future Consideration
If Go adds async/await or for I/O-heavy code, map to Rust's async:

```rust
async fn fetch_data() -> Result<String, Error> {
    // async I/O operations
}
```

### Benefits
- Better performance for I/O-bound operations
- Integration with Rust's async ecosystem
- More efficient than OS threads for many goroutines

## Implementation Priority

### High Priority (Correctness)
1. Statement-level closure capture
2. Interface method resolution
3. Package system support

### Medium Priority (Usability)
4. Goroutines and channels
5. Better error handling
6. Standard library mapping

### Low Priority (Optimization)
7. Escape analysis (Rc vs Arc)
8. Lifetime analysis
9. Optimization passes
10. Type inference improvements

### Future (Nice to Have)
11. Generics support
12. Async/await

## Testing Strategy

Each improvement should:
1. Start with comprehensive test cases in `tests/XFAIL/`
2. Implement incrementally with tests passing
3. Ensure no regression in existing tests
4. Document limitations and edge cases
5. Update README and AGENTS.md

## Conclusion

These improvements would transform go2rust from a correct but verbose transpiler into a production-ready tool generating idiomatic, performant Rust code. The key is to implement them incrementally, maintaining correctness at each step.