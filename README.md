# Go2Rust [![CI](https://github.com/tylerlaprade/go2rust/actions/workflows/ci.yml/badge.svg)](https://github.com/tylerlaprade/go2rust/actions/workflows/ci.yml)

The last Go program you'll ever need!

A conservative Go-to-Rust transpiler that prioritizes correctness over performance.

## Usage

```bash
# Build the transpiler
go build -o go2rust .

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

### Go Keywords Support

| Keyword | Status | Notes |
|---------|--------|-------|
| `break` | ❌ | |
| `case` | ❌ | |
| `chan` | ❌ | |
| `const` | ❌ | |
| `continue` | ❌ | |
| `default` | ❌ | |
| `defer` | ❌ | |
| `else` | ❌ | |
| `fallthrough` | ❌ | |
| `for` | ❌ | |
| `func` | ⚠️ | Basic functions with parameters and return types |
| `go` | ❌ | |
| `goto` | ❌ | |
| `if` | ❌ | |
| `import` | ⚠️ | Only fmt.Println |
| `interface` | ❌ | |
| `map` | ❌ | |
| `package` | ⚠️ | Only main package |
| `range` | ❌ | |
| `return` | ✅ | Single return values only |
| `select` | ❌ | |
| `struct` | ❌ | |
| `switch` | ❌ | |
| `type` | ❌ | |
| `var` | ❌ | |

### Standard Library Support

| Package | Status | Supported Functions |
|---------|--------|-------------------|
| `fmt` | ⚠️ | Println (with basic formatting) |
| Built-in | ⚠️ | println function |
| `strings` | ❌ | |
| `io` | ❌ | |
| `os` | ❌ | |
| `time` | ❌ | |
| `sync` | ❌ | |
| `net/http` | ❌ | |

## License

MIT License - see [LICENSE](LICENSE) file for details.
