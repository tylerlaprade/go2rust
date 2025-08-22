# Go2Rust [![CI](https://github.com/tylerlaprade/go2rust/actions/workflows/ci.yml/badge.svg)](https://github.com/tylerlaprade/go2rust/actions/workflows/ci.yml)

The last Go program you'll ever need!

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

## Features

### Type-Aware Transpilation

The transpiler uses Go's `go/types` package for accurate type information:

- Proper type checking across all files in a package
- Accurate detection of maps, slices, and other types
- Correct handling of method calls and type assertions
- No reliance on naming conventions or heuristics

### Error Handling

For unimplemented features, the transpiler generates TODO comments:

- Unhandled statements: `// TODO: Unhandled statement type: TypeName`
- Unhandled expressions: `/* TODO: Unhandled expression type: TypeName */`
- Unhandled type declarations: `// TODO: Unhandled type declaration: TypeName`

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
| **`fallthrough` - Fallthrough statements** | ❌ |
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
| └ String range | ❌ |
| └ Channel range | ❌ |
| **`return` - Return statements** | |
| └ Single return values | ✅ |
| └ Multiple return values | ✅ |
| └ Named returns | ❌ |
| **`select` - Select statements** | ❌ |
| **`struct` - Struct types** | |
| └ Struct definitions | ✅ |
| └ Struct literals | ✅ |
| └ Field access | ✅ |
| └ Embedded fields | 🚧 |
| └ Anonymous structs | ❌ |
| └ Struct tags | ✅ |
| **`switch` - Switch statements** | |
| └ Basic switch | ✅ |
| └ Type switch | ❌ |
| └ Fallthrough | ❌ |
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
| └ Closure variable capture | ⚠️ |
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
| **`strings` package** | |
| └ `strings.ToUpper` | ✅ |
| └ `strings.ToLower` | ✅ |
| └ `strings.TrimSpace` | ✅ |
| **`strconv` package** | |
| └ `strconv.Itoa` | ✅ |
| └ `strconv.Atoi` | ✅ |

| **`errors` package** | |
| └ `errors.New` | ✅ |
| **`sort` package** | |
| └ `sort.Strings` | ✅ |

### XFAIL Tests (Expected Failures)

The `tests/XFAIL/` directory contains tests for features not yet implemented. These tests:

- **Document the roadmap**: Each XFAIL test represents a planned feature
- **Enable TDD workflow**: Write the Go code you want to support, then implement the transpiler
- **Auto-promote when ready**: If an XFAIL test starts passing, it automatically moves to the main test suite
- **Fail CI on unexpected passes**: Prevents accidental feature implementation without proper review

### Enhanced Test Harness

The test runner (`./test.sh`) includes several advanced features:

- **Parallel execution**: Control with `-n/--jobs N` (default: CPU cores)
- **Timeout protection**: Set with `-t/--timeout TIME` (default: 60s per test)
- **Real-time output**: Use `-n 1` for sequential mode with live updates
- **Comprehensive reporting**: Shows passing, failing, and skipped tests
- **Auto-promotion**: XFAIL tests automatically move when they start passing

### Contributing XFAIL Tests

To add a new planned feature:

1. Create `tests/XFAIL/feature_name/main.go` with valid Go code
2. Run `./test.sh` - the test will be marked as "skip"
3. When the feature is implemented, the test will auto-promote to the main suite

### Test Determinism

**IMPORTANT**: All tests must produce deterministic output. The test infrastructure compares Go and Rust outputs byte-for-byte.

Common patterns to ensure determinism:

- **Map iteration**: Sort keys before iterating (see `tests/maps_basic/main.go`)
- **Goroutines**: Use proper synchronization (WaitGroup, channels)
- **Time/Random**: Use fixed values in tests

Note: Go 1.12+ prints maps in sorted key order with `fmt` functions, making map printing deterministic.
