# Go2Rust

The last Go program you'll ever need!

A conservative Go-to-Rust transpiler that prioritizes correctness over performance.

## Usage

```bash
# Build the transpiler
go build -o go2rust main.go

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
func main() {
    fmt.Println("Hello, World!")
}
```

**Output (Rust):**

```rust
fn main() {
    println!("Hello, World!");
}
```

## Architecture

- **parser**: Wraps Go's standard `go/parser` and `go/ast`
- **translator**: Converts Go AST to internal Rust representation
- **generator**: Generates Rust source code from internal representation

## Philosophy

This transpiler uses a "make it work first, optimize later" approach. Every Go pointer becomes `Arc<Mutex<Option<T>>>` initially, ensuring semantic correctness even if performance isn't optimal.

## License

MIT License - see [LICENSE](LICENSE) file for details.
