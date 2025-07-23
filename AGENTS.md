# Go2Rust Transpiler Project

## Project Overview

go2rust is a transpiler that converts Go code to Rust using a conservative "make it work first, optimize later" approach. Unlike c2rust which translates C's manual memory management directly to unsafe Rust, go2rust must bridge the semantic gap between Go's garbage collection and Rust's ownership model.

## Core Philosophy: Conservative Translation

### The Arc<Mutex<Option<T>>> Approach

Every Go pointer/reference type becomes `Arc<Mutex<Option<T>>>` in Rust:

- **Arc**: Handles Go's shared references between goroutines (reference counting)
- **Mutex**: Allows Go's concurrent mutation patterns
- **Option**: Represents Go's nil pointers

Example:

```go
// Go
type Person struct {
    Name string
    Age  int
}
p := &Person{Name: "Alice", Age: 30}
q := p  // Aliasing
```

```rust
// Initial Rust translation
let p = Arc::new(Mutex::new(Some(Person {
    name: String::from("Alice"),
    age: 30,
})));
let q = p.clone();  // Shared reference
```

### Why This Approach Works

1. **Semantic Preservation**: Go allows multiple references to mutate the same data
2. **Thread Safety**: Go pointers can be shared between goroutines
3. **Nil Safety**: Go pointers can be nil
4. **Working Code First**: Better to have slow, correct code than fast, broken code

## Implementation Status

### Phase 1: Hello World ✅

**Goal**: Transpile the simplest possible program

```go
package main
import "fmt"
func main() {
    fmt.Println("Hello, World!")
}
```

**Implemented**:

- Basic package structure → Rust module
- Import handling (just fmt)
- Function translation
- String literals
- Function parameters and return types
- Basic function calls
- String literal to String conversion (.to_string())

### Phase 2: Variables and Basic Types

**Goal**: Handle basic variable declarations and primitive types

```go
func main() {
    var x int = 42
    y := "hello"
    z := 3.14
    fmt.Println(x, y, z)
}
```

**Features needed**:

- Type inference for `:=`
- Basic type mapping (int → i32, string → String, etc.)
- Multiple arguments to fmt.Println (format string generation)

### Phase 3: Pointers and Mutation

**Goal**: Implement the Arc<Mutex<Option<>>> transformation

```go
func main() {
    x := &MyStruct{Value: 42}
    y := x  // Aliasing!
    y.Value = 100
    fmt.Println(x.Value)  // Prints 100
}
```

**Features needed**:

- Pointer type detection
- Arc<Mutex<Option<>>> wrapping
- Dereferencing (auto-generated `.lock().unwrap()` chains)
- Struct translation

### Phase 4: Functions and Methods

**Goal**: Handle function definitions and method receivers

```go
type Person struct {
    Name string
}

func (p *Person) Greet() {
    fmt.Println("Hello,", p.Name)
}

func NewPerson(name string) *Person {
    return &Person{Name: name}
}
```

### Phase 5: Goroutines and Basic Concurrency

**Goal**: Transform goroutines to threads

```go
func main() {
    x := &Counter{n: 0}
    go func() {
        x.n++
    }()
    time.Sleep(time.Second)
}
```

**Features needed**:

- `go` → `thread::spawn`
- Closure variable capture
- Basic time.Sleep mapping

### Phase 6: The Bootstrap Test! 🚀

**Goal**: go2rust can transpile itself

This is the ultimate validation - if the transpiler can transpile itself, we've handled enough of Go to claim success.

## Technical Decisions

### Type Mapping

| Go Type | Initial Rust Type | Optimized Rust Type |
|---------|------------------|-------------------|
| `*T` | `Arc<Mutex<Option<T>>>` | `&T`, `&mut T`, or `Box<T>` |
| `[]T` | `Arc<Mutex<Vec<T>>>` | `Vec<T>` or `&[T]` |
| `map[K]V` | `Arc<Mutex<HashMap<K,V>>>` | `HashMap<K,V>` |
| `interface{}` | `Arc<Mutex<Option<Box<dyn Any>>>>` | Specific types |
| `chan T` | `(Sender<T>, Receiver<T>)` | Same |

### Import Handling Strategy

1. **Direct transpilation**: For simple cases like `fmt.Println` → `println!`
2. **Use go/types**: For symbol resolution and type checking when needed
3. **Progressive addition**: Add stdlib support as needed
4. **FFI fallback**: For unsupported packages (future)

### Concurrency Detection

Simple initial approach:

```go
func hasGoroutines(file *ast.File) bool {
    // Scan for "go" statements or channel operations
    // If found, use Arc<Mutex<>> for everything
    // Otherwise, might use Rc<RefCell<>> (future optimization)
}
```

### Error Handling

Go's multiple return values:

```go
result, err := doSomething()
if err != nil {
    return nil, err
}
```

Becomes:

```rust
let result = do_something()?;
// OR
match do_something() {
    Ok(result) => result,
    Err(err) => return Err(err),
}
```

## Current Project Structure

```
go2rust/
├── main.go          # CLI entry point
├── transpile.go     # Direct Go AST to Rust code generation
├── tests/           # Test cases
│   ├── hello_world.go
│   ├── simple_functions/
│   │   ├── lib.go
│   │   └── test.go
│   └── README.md
├── test.sh          # Test runner (auto-discovers tests)
├── tests.bats       # BATS test suite (auto-generated)
├── go.mod
├── LICENSE          # MIT License
└── README.md
```

**Note**: We use Go's built-in `go/types` package for type analysis rather than building our own symbol table.

## Testing Strategy

### BATS Test Framework

- Automatic test discovery from `tests/` directory
- Semantic equivalence testing (Go output == Rust output)
- Support for stdin test cases via subdirectories

### Test Structure

```
tests/
├── test_name.go          # Simple test
├── test_with_input.go    # Test requiring stdin
└── test_with_input/      # Input files
    ├── case1.txt
    └── case2.txt
```

### Progressive Test Suite

1. Start with hello world ✅
2. Add features one at a time
3. Never break existing tests
4. Each phase builds on previous

### The Bootstrap Test

```bash
# The moment of truth!
./go2rust *.go > go2rust_v2.rs
rustc go2rust_v2.rs
./go2rust_v2 tests/hello_world.go
```

## Future Optimizations (Post-MVP)

After we have working transpilation:

1. **Remove unnecessary Arc**
   - Escape analysis to find non-escaping values
   - Single-threaded programs can use Rc

2. **Remove unnecessary Mutex**
   - Read-only shared data can use Arc<T>
   - Non-concurrent access doesn't need locks

3. **Remove unnecessary Option**
   - Non-nil pointers can drop Option
   - Static analysis of nil checks

4. **Lifetime introduction**
   - Replace Arc with references where possible
   - Infer lifetime parameters

## Known Limitations (v1)

- No `unsafe` package support
- No reflection
- No cgo
- Limited stdlib support
- No circular package dependencies
- No build tags

## Success Metrics

1. **Phase 1**: Can transpile hello world ✅
2. **Phase 2-3**: Can transpile basic programs
3. **Phase 4-5**: Can handle real Go patterns
4. **Phase 6**: Can transpile itself!
5. **Future**: Community can optimize output
