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
   - Uses one workspace wrapper policy across the root package and transpiled dependencies
   - Emits shared stdlib stand-ins in `vendor/go2rust_stdlib_stubs` so dependency crates agree on imported stdlib type identities

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
| └ Direct `break` in switch and type-switch cases | ✅ |
| **`case` - Case clauses** | |
| └ Switch cases | ✅ |
| └ Select cases | ✅ |
| └ Type switch cases | ✅ |
| └ Type switches over temporary call results | ✅ |
| **`chan` - Channel types** | ✅ |
| └ Channel struct fields with nil checks, send/receive, `len`, and `cap` | ✅ |
| **`const` - Constants** | |
| └ Basic constants | ✅ |
| └ String constants passed to `strings.Builder` | ✅ |
| └ Iota enumerations | ✅ |
| └ Complex iota expressions | ✅ |
| └ Named iota-backed enum types, including struct fields, untyped literal fields, and underlying const widths | ✅ |
| └ Named integer constants passed to function and method parameters | ✅ |
| └ Named integer constants in binary expressions with typed peers | ✅ |
| └ Cross-file package constants in compound assignments | ✅ |
| **`continue` - Continue statements** | ✅ |
| **`default` - Default clauses** | |
| └ Switch default | ✅ |
| └ Select default | ✅ |
| **`defer` - Defer statements** | ✅ |
| └ Methods with named returns and deferred result mutation | ✅ |
| └ Named return captures in deferred map writes | ✅ |
| └ Deferred field writes honor capture renames | ✅ |
| └ Deferred nested pointer field updates | ✅ |
| **`else` - Else clauses** | ✅ |
| **`fallthrough` - Fallthrough statements** | ✅ |
| **`for` - For loops** | |
| └ C-style for loops | ✅ |
| └ While-style loops | ✅ |
| └ Infinite loops | ✅ |
| └ Range loops (slice/map/string/channel, nil slices) | ✅ |
| └ Range value field access for pointer slices | ✅ |
| └ Range string values passed to string parameters | ✅ |
| └ Range struct values passed to function and method parameters | ✅ |
| └ Range over integers | ✅ |
| **`func` - Functions** | |
| └ Basic functions | ✅ |
| └ Multiple return values | ✅ |
| └ Method definitions | ✅ |
| └ Reassigned function and method parameters | ✅ |
| └ Tuple return reassignment to fields and parameters | ✅ |
| └ Method calls, including receiver self-calls | ✅ |
| └ Current receiver method calls with receiver-referencing arguments | ✅ |
| └ Method calls with function-literal arguments that capture the receiver | ✅ |
| └ Wrapped call results passed as method/function arguments | ✅ |
| └ Typed nil arguments passed to package functions | ✅ |
| └ Function-valued struct field calls | ✅ |
| └ Method selectors returning function values stay method calls | ✅ |
| └ Named function type conversions | ✅ |
| └ Cross-package named function type conversions | ✅ |
| └ Function type aliases with imported interface parameters | ✅ |
| └ Function literals/closures with scoped nested captures | ✅ |
| └ Function variables and higher-order function values | ✅ |
| └ Instantiated generic function type aliases | ✅ |
| └ Focused type-parameter constraints in concrete helper signatures such as `S ~[]T, T ~string` | ✅ |
| └ Assignment from function return values | ✅ |
| └ Variadic functions | ✅ |
| └ Cross-file variadic function calls | ✅ |
| **`go` - Goroutines** | ✅ |
| └ Worker pool pattern with channels | ✅ |
| └ Method receiver captures with deferred receiver calls | ✅ |
| └ Function-typed parameter captures | ✅ |
| **`goto` - Goto statements** | ✅ (basic top-level label patterns) |
| **`if` - If statements** | |
| └ Basic if | ✅ |
| └ If with init statement | ✅ |
| └ If-else chains | ✅ |
| **`import` - Imports** | |
| └ Single imports | ✅ |
| └ Multiple imports | ✅ |
| └ Package aliases | ✅ |
| └ Blank imports | ✅ |
| └ Stdlib type signatures | ✅ |
| └ Stdlib method stubs from selector type information | ✅ |
| └ Stdlib pointer method calls from indexed and range receivers | ✅ |
| └ Stdlib package function/constant/variable stubs | ✅ |
| └ Stdlib package variadic stub calls | ✅ |
| └ Stdlib package stub calls from wrapped range variables | ✅ |
| └ Stdlib typed constants as indexes | ✅ |
| └ Shared stdlib stubs across transpiled dependency crates | ✅ |
| └ Stdlib concrete values passed to stdlib interface parameters | ✅ |
| └ Stdlib interface map range keys passed to interface parameters | ✅ |
| └ Stdlib interface field copies | ✅ |
| └ Stdlib pointer field stubs without nested wrappers | ✅ |
| **`interface` - Interface types** | |
| └ Interface definitions | ✅ |
| └ Empty interface{} | ✅ |
| └ Named `any` returns default to nil | ✅ |
| └ Selector fields returned as `any` preserve the interface handle | ✅ |
| └ Existing `any` values assigned to `any` fields preserve the interface handle | ✅ |
| └ Empty struct literals with `any` fields default to nil | ✅ |
| └ Concurrent `any` and local-interface fields/globals use `Send + Sync` trait objects | ✅ |
| └ Local interface values clone, store, return, compare, and work with `slices.Contains` via generated trait-object helpers | ✅ |
| └ Interface comparisons against concrete package-global pointers | ✅ |
| └ Typed constants passed to local interface parameters | ✅ |
| └ Variadic `any` arguments | ✅ |
| └ Interface implementations | ✅ |
| └ Type assertions | ✅ |
| └ `any(x)` conversions feeding type assertions | ✅ |
| └ Static `any(x).(interface{...})` assertions when TypeInfo proves implementation | ✅ |
| └ Stdlib concrete pointer literals and values returned as stdlib interface types | ✅ |
| └ Imported transpiled interfaces implemented by current-package concrete types | ✅ |
| └ Boxed error values asserted back to `error` | ✅ |
| └ Wrapped `error` arguments and sentinel comparisons | ✅ |
| └ Type switches, including nil and selector pointer cases | ✅ |
| **`map` - Map types** | |
| └ Map types | ✅ |
| └ Map literals | ✅ |
| └ Map operations (insert, delete) | ✅ |
| └ Delete through selector map fields | ✅ |
| └ Selector map fields in lookup, comma-ok, and range contexts | ✅ |
| └ Map access with existence check | ✅ |
| └ Comma-ok map access with typed zero values | ✅ |
| └ Wrapped call results as map lookup keys | ✅ |
| └ Pointer keys in map literals and lookups | ✅ |
| └ Pointer keys and values in `make(map)` plus slice-range pointer key lookups | ✅ |
| └ Named integer values in map literals, writes, lookups, and comma-ok zero values | ✅ |
| └ Struct keys with named integer fields across split modules | ✅ |
| └ Append results assigned into map slice values | ✅ |
| └ Nested range over map slice values | ✅ |
| └ Map iteration (for range) | ✅ |
| └ Map printing (sorted keys, Go 1.12+) | ✅ |
| **`package` - Packages** | |
| └ Main package | ✅ |
| └ Library packages | ✅ |
| └ Package-level variable initialization | ✅ |
| └ Package-level error initialization | ✅ |
| └ Package-level map literal initialization | ✅ |
| └ Package-level named slice declarations | ✅ |
| └ Package-level pointer globals initialized from constructor calls | ✅ |
| └ Exported package globals from transpiled dependency crates keep their Go names and public visibility | ✅ |
| └ Method calls on exported pointer globals from transpiled dependency crates | ✅ |
| └ Init functions | ✅ |
| └ Multi-file packages with cross-file types, methods, maps, slices, and function variables | 🚧 |
| └ Package-scoped generated helpers for multi-file stdlib/time/context types | ✅ |
| └ Workspace-wide wrapper selection for transpiled external packages | ✅ |
| **`range` - Range clauses** | |
| └ Array/slice range, including nil slices | ✅ |
| └ Map range | ✅ |
| └ String range | ✅ |
| └ Channel range | ✅ |
| **`return` - Return statements** | |
| └ Single return values | ✅ |
| └ Multiple return values | ✅ |
| └ Named returns | ✅ |
| └ Slice and map literal returns | ✅ |
| **`select` - Select statements** | ✅ |
| **`struct` - Struct types** | |
| └ Struct definitions | ✅ |
| └ Struct literals | ✅ |
| └ Comparable struct literals in equality expressions | ✅ |
| └ Channel fields in struct literals | ✅ |
| └ Selector-qualified struct literals (`pkg.Type{}`) | ✅ |
| └ Stdlib struct field stubs from selector type information | ✅ |
| └ Field access | ✅ |
| └ Owned selector field returns and arguments | ✅ |
| └ Owned selector field short declarations | ✅ |
| └ Embedded fields | ✅ |
| └ Embedded method promotion from transpiled dependency packages | ✅ |
| └ Anonymous structs | ✅ |
| └ Nested structs/slices/maps/interfaces and anonymous struct function boundaries | ✅ |
| └ Struct fields containing map values with `any` contents | ✅ |
| └ Struct tags | ✅ |
| **`switch` - Switch statements** | |
| └ Basic switch | ✅ |
| └ Type switch | ✅ |
| └ Fallthrough | ✅ |
| **`type` - Type definitions** | |
| └ Struct types | ✅ |
| └ Type aliases | ✅ |
| └ Type definitions | ✅ |
| └ Go type names that collide with Rust prelude types are escaped in generated Rust | ✅ |
| └ Named scalar newtypes usable across generated modules | ✅ |
| └ Named scalar equality comparisons | ✅ |
| └ Instantiated generic function type aliases | ✅ |
| └ Numeric conversions from literals and expressions | ✅ |
| └ Numeric conversions from named integer values | ✅ |
| └ Named slice type definitions in methods, parameters, ranges, indexing, append, and field method arguments | ✅ |
| └ Named slice type definitions in package globals | ✅ |
| └ Interface types | ✅ |
| **`var` - Variable declarations** | |
| └ Basic var declarations | ✅ |
| └ Package-qualified local zero values | ✅ |
| └ Package-level declarations and initializer order | ✅ |
| └ Package-level fixed and inferred array declarations | ✅ |
| └ Short declarations (:=) | ✅ |
| └ Selector slice field short declarations | ✅ |
| └ Blank identifier (_) | ✅ |
| **Arrays & Slices** | |
| └ Fixed arrays | ✅ |
| └ Fixed array literals with omitted trailing zero values | ✅ |
| └ Fixed array zero values above Rust's built-in `Default` array sizes | ✅ |
| └ Slices | ✅ |
| └ Array/slice literal elements from wrapped call results | ✅ |
| └ Array/slice/map literals passed to package functions | ✅ |
| └ Slice operations | ✅ |
| └ Append into indexed slice elements inside arrays/slices | ✅ |
| └ Assignment into nested indexed slice elements | ✅ |
| └ `cap` on wrapped slice fields | ✅ |
| └ Range over wrapped call results and pointer-to-array targets | ✅ |
| └ Named integer values as array/slice indexes | ✅ |
| └ Parallel slice element swaps | ✅ |
| └ Parallel slice-expression assignments into wrapped fields | ✅ |
| └ Concurrent indexed byte compound assignments | ✅ |
| **Operators** | |
| └ Binary operators (+, -, *, /, etc.) | ✅ |
| └ Complex nested expressions with function calls, indexing, fields, pointers, assertions, and channel receives | ✅ |
| └ Unary NOT and conditions on wrapped bools | ✅ |
| └ Concurrent binary comparisons with bare `len`/`cap` operands | ✅ |
| └ Binary `len`/`cap` operands with typed `int` peers | ✅ |
| └ Untyped constants compared with named and typed integer operands | ✅ |
| └ Cross-file named integer conversions from wrapped calls | ✅ |
| └ Assignment operators (=, +=, etc.) | ✅ |
| └ Increment/decrement (++, --) | ✅ |
| └ Address-of (&) | ✅ |
| └ Dereference (*) | ✅ |
| **Pointers** | |
| └ Pointer types (*T) | ✅ |
| └ Address-of operator | ✅ |
| └ Dereference operator | ✅ |
| └ Pointer aliasing | ✅ |
| └ Pointer receiver nil comparisons and pointer variable call arguments | ✅ |
| └ Pointer receivers stored in pointer fields | ✅ |
| └ Address-of local values passed to pointer parameters | ✅ |
| └ Address-of selector fields passed to pointer parameters | ✅ |
| └ Address-of slice elements with named integer indexes | ✅ |
| └ Address-of struct fields and slice elements | ✅ |
| └ Declared pointers to slice elements | ✅ |
| └ Field access through ranged slice pointers | ✅ |
| **String Operations** | |
| └ String concatenation (+) | ✅ |
| └ String += operator | ✅ |
| └ Returning string constants from string functions | ✅ |
| └ String comparisons | ✅ |
| └ Byte comparisons with character literals | ✅ |
| └ Raw string literals | ✅ |
| └ `len` on string literals in slice bounds | ✅ |
| └ `[]byte`/`[]rune` conversions from selector expressions | ✅ |
| **Closures & Anonymous Functions** | |
| └ Function literals | ✅ |
| └ Closure variable capture | ✅ |
| └ Recursive closure assignment through function variables | ✅ |
| └ Anonymous function calls | ✅ |
| └ Cross-file function variables | ✅ |
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
| └ `len`/`cap` passed to Go `int` parameters | ✅ |
| └ `len`/`cap` short declarations as Go `int` values | ✅ |
| └ `append` | ✅ |
| └ `copy` | ✅ |
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
| └ `%w` error formatting | ✅ |
| └ `%T` type-name formatting in errors | ✅ |
| └ Dynamic-format `panic(fmt.Errorf(...))` | ✅ |
| └ `%+v` debug and `%#x` alternate hex formatting | ✅ |
| └ `%x`/`%X` byte formatting | ✅ |
| **`strings` package** | |
| └ `strings.ToUpper` | ✅ |
| └ `strings.ToLower` | ✅ |
| └ `strings.TrimSpace` | ✅ |
| └ `strings.Title` | ✅ |
| └ `strings.Contains` | ✅ |
| └ `strings.Index` / `strings.LastIndex` / `strings.IndexAny` | ✅ |
| └ `strings.Count` | ✅ |
| └ `strings.Compare` / `strings.Cut` | ✅ |
| └ `strings.HasPrefix` / `strings.HasSuffix` | ✅ |
| └ `strings.Split` / `strings.Join` / `strings.Fields` | ✅ |
| └ `strings.Replace` / `strings.ReplaceAll` | ✅ |
| └ `strings.Repeat` | ✅ |
| └ `strings.EqualFold` | ✅ |
| └ `strings.Trim` / `strings.TrimLeft` / `strings.TrimRight` | ✅ |
| **`strconv` package** | |
| └ `strconv.Itoa` | ✅ |
| └ `strconv.Atoi` | ✅ |
| └ `strconv.FormatFloat` / `strconv.FormatInt` | ✅ |
| **`math` package** | |
| └ `math.Pi` / `math.E` | ✅ |
| └ `math.Sqrt` | ✅ |
| └ `math.Pow` | ✅ |
| └ `math.Max` / `math.Min` | ✅ |
| **`math/rand` package** | |
| └ `rand.Seed`, `rand.Intn`, `rand.Float64` | ✅ |
| **`encoding/json` package** | |
| └ `json.Marshal` for structs with exported basic fields | ✅ |
| **`encoding/base64` package** | |
| └ `base64.StdEncoding.EncodeToString` | ✅ |
| └ `base64.StdEncoding.DecodeString` | ✅ |
| **`crypto/sha256` package** | |
| └ `sha256.Sum256` | ✅ |
| **`net/url` package** | |
| └ `url.Parse` | ✅ |
| **`regexp` package** | |
| └ `regexp.MustCompile` + basic `FindAllString` (`\d+` and literal matches) | ✅ |
| **`reflect` package** | |
| └ `reflect.TypeOf` struct field metadata and `StructTag.Get` | ✅ |
| └ Pointer conversions to reflected struct header stand-ins | ✅ |
| **`unsafe` package** | |
| └ `unsafe.Sizeof` / `unsafe.Alignof` for Rust representation layout | ✅ |
| └ `unsafe.Sizeof` in integer comparisons and conversions | ✅ |
| └ Named `unsafe.Pointer` definitions round-trip through `uintptr` and `any` | ✅ |
| **`errors` package** | |
| └ `errors.New` | ✅ |
| └ Package-level `errors.New` values | ✅ |
| └ Custom error types | ✅ |
| └ Type assertions on errors | ✅ |
| └ Error handle arguments and equality comparisons | ✅ |
| **`flag` package** | |
| └ `flag.String` / `flag.Parse` default values | ✅ |
| **`time` package** | |
| └ `time.Unix`, `Time.UTC`, `Time.Add`, `Time.Unix`, `Time.UnixNano` | ✅ |
| └ `time.NewTimer` plus `Timer.C` receive and `Timer.Stop` | ✅ |
| └ `time.After` timeout channels | ✅ |
| └ `time.NewTicker` plus `Ticker.C` receive and `Ticker.Stop` | ✅ |
| └ `time.Tick` periodic channels | ✅ |
| **`context` package** | |
| └ `context.Background`, `context.WithTimeout`, `context.WithCancelCause`, `Context.Done`, `Context.Err`, cancel funcs | ✅ |
| **`os` package** | |
| └ `os.Args` read access | ✅ |
| └ `os.Create` / `os.Remove` plus file `WriteString` / `Close` | ✅ |
| **`sort` package** | |
| └ `sort.Strings` | ✅ |
| **`slices` package** | |
| └ `slices.Sort` / `slices.SortFunc` / `slices.Contains` | ✅ |
| **`sync` package** | |
| └ `sync.WaitGroup` | ✅ |
| └ Zero-value `sync.WaitGroup` struct fields | ✅ |
| └ `sync.Mutex` | ✅ |
| └ `sync.Once` | ✅ |
| **`sync/atomic` package** | |
| └ `atomic.AddInt64` / `atomic.LoadInt64` | ✅ |
| **`strings` (Builder)** | |
| └ `strings.Builder` | ✅ |
| └ `Builder.WriteString` with string constants | ✅ |

### XFAIL Tests (Expected Failures)

The `tests/XFAIL/` directory contains tests for features not yet implemented. These tests:

- **Document the roadmap**: Each XFAIL test represents a planned feature
- **Enable TDD workflow**: Write the Go code you want to support, then implement the transpiler
- **Auto-promote when ready**: If an XFAIL test starts passing, it automatically moves to the main test suite
- **Fail CI on unexpected passes**: Prevents accidental feature implementation without proper review

### Contributing XFAIL Tests

To report an unimplemented or broken feature, create a pull request adding `tests/XFAIL/feature_name/main.go` with compilable, deterministic Go code.
