# Go2Rust [![CI](https://github.com/tylerlaprade/go2rust/actions/workflows/ci.yml/badge.svg)](https://github.com/tylerlaprade/go2rust/actions/workflows/ci.yml)

The last Go program you'll ever need!

<p align="center">
  <img alt="gopher2ferris" src="gopher2ferris.png" /></a>
    <i>Artwork by <a href="https://linktr.ee/bwh.art">Bonnie Hansen</a></i>
</p>

A conservative Go-to-Rust transpiler that prioritizes correctness over performance.

## Usage

```bash
# Build the transpiler
go build -o go2rust ./go

# Transpile a Go file
./go2rust input.go > output.rs

# Run tests
./test.sh
```

### External Package Handling

Go2Rust provides four modes for handling external package imports:

1. **`transpile` (default)**: Recursively transpiles all dependencies to Rust
   - Pure Rust output with no Go runtime dependency
   - Currently in development

2. **`stub`**: Generates stub implementations for external packages
   - Creates placeholder Rust modules with helpful TODO comments
   - Allows you to manually implement or use Rust equivalents
   - Useful when automatic transpilation fails or when you want custom implementations
   - Stub files are generated in `vendor/` directory

3. **`ffi`**: Generates FFI bridge to call Go libraries from Rust
   - Keeps Go packages as-is and generates bindings
   - Useful for packages with cgo or complex dependencies
   - Currently in development

4. **`none`**: Fails if external packages are imported
   - Useful for simple, self-contained programs
   - Ensures no external dependencies

## Example

**Input (Go):**

```go
package main

import "fmt"

func GetGreeting() string {
    return "Hello, World!"
}

func GetYear() int {
    return 2024
}

func main() {
    fmt.Println(GetGreeting())
    println(GetYear())
}
```

**Output (Rust) - Single-threaded code:**

```rust
use std::rc::Rc;
use std::cell::RefCell;

fn get_greeting() -> Rc<RefCell<Option<String>>> {
    return Rc::new(RefCell::new(Some("Hello, World!".to_string())));
}

fn get_year() -> Rc<RefCell<Option<i32>>> {
    return Rc::new(RefCell::new(Some(2024)));
}

fn main() {
    println!("{}", (*get_greeting().borrow().as_ref().unwrap()));
    println!("{:?}", (*get_year().borrow().as_ref().unwrap()));
}
```

When the transpiler detects concurrency (goroutines, channels, or async stdlib calls), it automatically uses `Arc<Mutex<Option<T>>>` instead for thread safety.

## Philosophy

This transpiler uses a "make it work first, optimize later" approach. **EVERY Go value** is wrapped for safety, but the wrapper type depends on concurrency needs:

- **Single-threaded code**: Uses `Rc<RefCell<Option<T>>>` for better performance
- **Concurrent code**: Uses `Arc<Mutex<Option<T>>>` for thread safety

This ensures semantic correctness for ANY Go program, even edge cases like taking the address of function parameters. The generated code is verbose but correct. Users can optimize later.

## Progress Tracking

### Go Keywords (25 total)

| Keyword | Status |
|---------|--------|
| **`break` - Break statements** | ✅ |
| **`case` - Case clauses** | |
| └ Switch cases | ✅ |
| └ Select cases | ❌ |
| └ Type switch cases | ❌ |
| **`chan` - Channel types** | ❌ |
| **`const` - Constants** | |
| └ Basic constants | ✅ |
| └ Iota enumerations | ✅ |
| └ Complex iota expressions | ✅ |
| **`continue` - Continue statements** | ✅ |
| **`default` - Default clauses** | |
| └ Switch default | ✅ |
| └ Select default | ❌ |
| **`defer` - Defer statements** | ✅ |
| **`else` - Else clauses** | ✅ |
| **`fallthrough` - Fallthrough statements** | ✅ |
| **`for` - For loops** | |
| └ C-style for loops | ✅ |
| └ While-style loops | ✅ |
| └ Infinite loops | ✅ |
| └ Range loops | ✅ |
| **`func` - Functions** | |
| └ Basic functions | ✅ |
| └ Multiple return values | ✅ |
| └ Method definitions | ✅ |
| └ Method calls | ✅ |
| └ Function literals/closures | ✅ |
| └ Variadic functions | ✅ |
| **`go` - Goroutines** | ✅ |
| **`goto` - Goto statements** | ❌ |
| **`if` - If statements** | |
| └ Basic if | ✅ |
| └ If with init statement | ✅ |
| └ If-else chains | ✅ |
| **`import` - Imports** | |
| └ Single imports | ✅ |
| └ Multiple imports | ✅ |
| └ Package aliases | ❌ |
| └ Blank imports | ❌ |
| **`interface` - Interface types** | |
| └ Interface definitions | ✅ |
| └ Empty interface{} | ✅ |
| └ Interface implementations | ❌ |
| └ Type assertions | ✅ |
| └ Type switches | ❌ |
| **`map` - Map types** | |
| └ Map types | ✅ |
| └ Map literals | ✅ |
| └ Map operations (insert, delete) | ✅ |
| └ Map access with existence check | ✅ |
| └ Map iteration (for range) | ✅ |
| └ Map printing (sorted keys, Go 1.12+) | ✅ |
| **`package` - Packages** | |
| └ Main package | ✅ |
| └ Library packages | ✅ |
| └ Multi-file packages | ❌ |
| **`range` - Range clauses** | |
| └ Array/slice range | ✅ |
| └ Map range | ✅ |
| └ String range | ✅ |
| └ Channel range | ✅ |
| **`return` - Return statements** | |
| └ Single return values | ✅ |
| └ Multiple return values | ✅ |
| └ Named returns | ✅ |
| **`select` - Select statements** | ❌ |
| **`struct` - Struct types** | |
| └ Struct definitions | ✅ |
| └ Struct literals | ✅ |
| └ Field access | ✅ |
| └ Embedded fields | ✅ |
| └ Anonymous structs | 🚧 |
| └ Struct tags | ✅ |
| **`switch` - Switch statements** | |
| └ Basic switch | ✅ |
| └ Type switch | ✅ |
| └ Fallthrough | ✅ |
| **`type` - Type definitions** | |
| └ Struct types | ✅ |
| └ Type aliases | ✅ |
| └ Type definitions | ✅ |
| └ Interface types | ✅ |
| **`var` - Variable declarations** | |
| └ Basic var declarations | ✅ |
| └ Short declarations (:=) | ✅ |
| └ Blank identifier (_) | ✅ |
| **Arrays & Slices** | |
| └ Fixed arrays | ✅ |
| └ Slices | ✅ |
| └ Slice operations | ✅ |
| **Operators** | |
| └ Binary operators (+, -, *, /, etc.) | ✅ |
| └ Assignment operators (=, +=, etc.) | ✅ |
| └ Increment/decrement (++, --) | ✅ |
| └ Address-of (&) | ✅ |
| └ Dereference (*) | ✅ |
| **Pointers** | |
| └ Pointer types (*T) | ✅ |
| └ Address-of operator | ✅ |
| └ Dereference operator | ✅ |
| └ Pointer aliasing | ✅ |
| **String Operations** | |
| └ String concatenation (+) | ✅ |
| └ String += operator | ✅ |
| └ String comparisons | ✅ |
| **Closures & Anonymous Functions** | |
| └ Function literals | ✅ |
| └ Closure variable capture | ✅ |
| └ Anonymous function calls | ✅ |
| └ Capture analysis framework | ✅ |
| **`defer` - Defer statements** | |
| └ Basic defer | ✅ |
| └ Multiple defers (LIFO order) | ✅ |
| └ Defer with closures | ✅ |
| └ Defer stack management | ✅ |

### Standard Library Functions

| Function | Status |
|----------|--------|
| **Built-in functions** | |
| └ `println` | ✅ |
| └ `len` | ✅ |
| └ `cap` | ✅ |
| └ `append` | ✅ |
| └ `make` | ✅ |
| └ `delete` | ✅ |
| └ `new` | ✅ |
| **`fmt` package** | |
| └ `fmt.Println` | ✅ |
| └ `fmt.Printf` | ✅ |
| └ `fmt.Sprintf` | ✅ |
| └ `fmt.Errorf` | ✅ |
| └ `fmt.Fprintln` | ✅ |
| └ `fmt.Fprintf` | ✅ |
| **`strings` package** | |
| └ `strings.ToUpper` | ✅ |
| └ `strings.ToLower` | ✅ |
| └ `strings.TrimSpace` | ✅ |
| **`strconv` package** | |
| └ `strconv.Itoa` | ✅ |
| └ `strconv.Atoi` | ✅ |

| **`errors` package** | |
| └ `errors.New` | ✅ |
| └ Custom error types | ✅ |
| └ Type assertions on errors | ✅ |
| **`sort` package** | |
| └ `sort.Strings` | ✅ |

### XFAIL Tests (Expected Failures)

The `tests/XFAIL/` directory contains tests for features not yet implemented. These tests:

- **Document the roadmap**: Each XFAIL test represents a planned feature
- **Enable TDD workflow**: Write the Go code you want to support, then implement the transpiler
- **Auto-promote when ready**: If an XFAIL test starts passing, it automatically moves to the main test suite
- **Fail CI on unexpected passes**: Prevents accidental feature implementation without proper review

### Contributing XFAIL Tests

To report an unimplemented or broken feature, create a pull request adding `tests/XFAIL/feature_name/main.go` with compilable, deterministic Go code.
