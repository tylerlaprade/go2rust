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

**Output (Rust):**

```rust
use std::sync::{Arc, Mutex};

fn get_greeting() -> Arc<Mutex<Option<String>>> {
    return Arc::new(Mutex::new(Some("Hello, World!".to_string())));
}

fn get_year() -> Arc<Mutex<Option<i32>>> {
    return Arc::new(Mutex::new(Some(2024)));
}

fn main() {
    println!("{}", (*get_greeting().lock().unwrap().as_ref().unwrap()));
    println!("{:?}", (*get_year().lock().unwrap().as_ref().unwrap()));
}
```

Note: The actual output is more verbose than shown here due to our conservative wrapping approach. Every value is wrapped in `Arc<Mutex<Option<T>>>` to ensure correctness.

## Philosophy

This transpiler uses a "make it work first, optimize later" approach. **EVERY Go value** becomes `Arc<Mutex<Option<T>>>` - no exceptions. This includes:

- All variables (local, global)
- All function parameters
- All return values
- All struct fields
- All intermediate expressions

This ensures semantic correctness for ANY Go program, even edge cases like taking the address of function parameters. The generated code is verbose but correct. Users can optimize later.

### Recent Progress

- Fixed string mutation handling with proper `as_mut()` usage
- Added `.clone()` for extracting owned values from wrapped types
- Improved assignment handling for both LHS and RHS contexts
- All pointer operations now working correctly

## Progress Tracking

### Go Keywords

| Keyword | Status |
|---------|--------|
| **`func` - Functions** | |
| └ Basic functions | ✅ |
| └ Multiple return values | ✅ |
| └ Method definitions | ❌ |
| └ Method calls | ❌ |
| **`import` - Imports** | |
| └ Single imports | ✅ |
| └ Multiple imports | ✅ |
| └ Package aliases | ❌ |
| **`package` - Packages** | |
| └ Main package | ✅ |
| └ Library packages | ✅ |
| └ Package aliases | ❌ |
| **`return` - Return statements** | |
| └ Single return values | ✅ |
| └ Multiple return values | ✅ |
| **`struct` - Struct types** | |
| └ Struct definitions | ✅ |
| └ Struct literals | ✅ |
| └ Field access | ✅ |
| └ Embedded fields | ❌ |
| **`type` - Type definitions** | |
| └ Struct types | ✅ |
| └ Type aliases | ❌ |
| └ Interface types | ❌ |
| **`var` / `const` - Declarations** | |
| └ Basic var declarations | ✅ |
| └ Short declarations (:=) | ✅ |
| └ Constants (const) | ✅ |
| └ Blank identifier (_) | ✅ |
| **`for` - For loops** | |
| └ C-style for loops | ✅ |
| └ Range loops | ✅ |
| └ Infinite loops | ✅ |
| └ Break statements | ✅ |
| └ Continue statements | ✅ |
| **`switch` - Switch statements** | |
| └ Basic switch | ✅ |
| └ Type switch | ❌ |
| **`map` - Maps** | |
| └ Map types | ✅ |
| └ Map literals | ✅ |
| └ Map operations | ✅ |
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

### Standard Library Functions

| Function | Status |
|----------|--------|
| **Built-in functions** | |
| └ `println` | ✅ |
| └ `len` | ✅ |
| └ `cap` | ✅ |
| └ `append` | ✅ |
| └ `make` | ✅ |
| **`fmt` package** | |
| └ `fmt.Println` | ✅ |
| └ `fmt.Printf` | ❌ |
| └ `fmt.Sprintf` | ❌ |
| **`strings` package** | |
| └ `strings.ToUpper` | ✅ |
| └ `strings.ToLower` | ✅ |
| └ `strings.TrimSpace` | ✅ |
| **`strconv` package** | |
| └ `strconv.Itoa` | ✅ |
| └ `strconv.Atoi` | ❌ |
| **Memory functions** | |
| └ `new` | ✅ |

## Test Suite

### Test Categories

- **Basic Language Features**: Variables, types, operators, control flow
- **Functions**: Basic functions, multiple returns, parameter handling  
- **Data Structures**: Arrays, slices, maps, structs, pointers
- **Standard Library**: fmt, strings, strconv, builtin functions
- **Concurrency**: Goroutines, channels, select statements (planned)
- **Advanced Features**: Interfaces, generics, error handling (planned)

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
