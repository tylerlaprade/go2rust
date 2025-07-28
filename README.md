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
fn get_greeting() -> String {
    return "Hello, World!".to_string();
}

fn get_year() -> i32 {
    return 2024;
}

fn main() {
    println!("{}", get_greeting());
    println!("{:?}", get_year());
}
```

## Philosophy

This transpiler uses a "make it work first, optimize later" approach. Every Go pointer becomes `Arc<Mutex<Option<T>>>` initially, ensuring semantic correctness even if performance isn't optimal.

## Progress Tracking

### Go Keywords

| Keyword | Status |
|---------|--------|
| **`func` - Functions** | |
| └ Basic functions | ✅ |
| └ Method definitions | ❌ |
| └ Method calls | ❌ |
| **`import` - Imports** | |
| └ Single imports | ✅ |
| └ Multiple imports | ✅ |
| └ Package aliases | ❌ |
| **`package` - Packages** | |
| └ Main package | ✅ |
| └ Other packages | ❌ |
| **`return` - Return statements** | |
| └ Single return values | ✅ |
| └ Multiple return values | ❌ |
| **`struct` - Struct types** | |
| └ Struct definitions | ❌ |
| └ Struct literals | ❌ |
| └ Field access | ❌ |
| **`type` - Type definitions** | |
| └ Struct types | ❌ |
| └ Type aliases | ❌ |
| **`var` - Variable declarations** | |
| └ Basic var declarations | ✅ |
| └ Short declarations (:=) | ✅ |
| **`for` - For loops** | |
| └ C-style for loops | ✅ |
| └ Range loops | ❌ |
| └ Infinite loops | ✅ |
| **Operators** | |
| └ Binary operators (+, -, *, /, etc.) | ✅ |
| └ Assignment operators (=, +=, etc.) | ✅ |
| └ Increment/decrement (++, --) | ✅ |

### Standard Library Functions

| Function | Status |
|----------|--------|
| **Built-in functions** | |
| └ `println` | ❌ |
| └ `len` | ❌ |
| └ `append` | ❌ |
| **`fmt` package** | |
| └ `fmt.Println` | ✅ |
| └ `fmt.Printf` | ❌ |
| **`strings` package** | |
| └ `strings.ToUpper` | ❌ |
| └ `strings.ToLower` | ❌ |
| └ `strings.TrimSpace` | ✅ |
| **`strconv` package** | |
| └ `strconv.Itoa` | ❌ |

## XFAIL Tests (Expected Failures)

The `tests/XFAIL/` directory contains tests for features not yet implemented. These tests:

- **Document the roadmap**: Each XFAIL test represents a planned feature
- **Enable TDD workflow**: Write the Go code you want to support, then implement the transpiler
- **Auto-promote when ready**: If an XFAIL test starts passing, it automatically moves to the main test suite
- **Fail CI on unexpected passes**: Prevents accidental feature implementation without proper review

### Contributing XFAIL Tests

To add a new planned feature:

1. Create `tests/XFAIL/feature_name/main.go` with valid Go code
2. Run `./test.sh` - the test will be marked as "skip"
3. When the feature is implemented, the test will auto-promote to the main suite

## License

MIT License - see [LICENSE](LICENSE) file for details.
