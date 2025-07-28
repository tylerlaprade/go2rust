# Test Organization Strategy for go2rust

## Directory Structure

```
tests/
├── 01_basics/              # Phase 1: Basic language features
│   ├── hello_world.go      # Simplest program
│   ├── println_builtin.go  # Builtin println
│   └── fmt_println.go      # fmt.Println
│
├── 02_types/               # Phase 2: Type system
│   ├── primitives/
│   │   ├── int.go          # int, int8, int16, int32, int64
│   │   ├── uint.go         # uint, uint8, uint16, uint32, uint64
│   │   ├── float.go        # float32, float64
│   │   ├── bool.go         # boolean operations
│   │   ├── string.go       # string operations
│   │   └── rune_byte.go    # rune and byte types
│   │
│   ├── composite/
│   │   ├── array.go        # Fixed-size arrays
│   │   ├── slice.go        # Dynamic slices
│   │   ├── map.go          # Hash maps
│   │   ├── struct.go       # Basic structs
│   │   └── pointer.go      # Pointer basics
│   │
│   └── type_inference.go   # var x = 42, x := 42
│
├── 03_control_flow/        # Control structures
│   ├── if_else.go          # if, else if, else
│   ├── for_loop.go         # for loops (all variants)
│   ├── range.go            # for range loops
│   ├── switch.go           # switch statements
│   ├── break_continue.go   # break, continue
│   └── goto.go             # goto and labels
│
├── 04_functions/           # Functions and methods
│   ├── basic_func.go       # Simple functions
│   ├── multiple_return.go  # Multiple return values
│   ├── named_return.go     # Named return values
│   ├── variadic.go         # Variadic functions
│   ├── methods.go          # Methods on types
│   ├── closures.go         # Anonymous functions
│   └── defer.go            # defer statements
│
├── 05_pointers/            # Pointer operations (Arc<Mutex<Option<T>>>)
│   ├── address_of.go       # & operator
│   ├── dereference.go      # * operator
│   ├── nil_check.go        # nil pointer handling
│   ├── pointer_alias.go    # Multiple refs to same data
│   └── pointer_mutation.go # Mutation through pointers
│
├── 06_concurrency/         # Goroutines and channels
│   ├── goroutine.go        # Basic goroutine
│   ├── channel.go          # Channel operations
│   ├── buffered_chan.go    # Buffered channels
│   ├── select.go           # select statement
│   ├── mutex.go            # sync.Mutex
│   └── waitgroup.go        # sync.WaitGroup
│
├── 07_interfaces/          # Interface system
│   ├── basic_interface.go  # Simple interface
│   ├── empty_interface.go  # interface{}
│   ├── type_assertion.go   # Type assertions
│   ├── type_switch.go      # Type switches
│   └── embedding.go        # Interface embedding
│
├── 08_error_handling/      # Error patterns
│   ├── error_return.go     # if err != nil pattern
│   ├── custom_error.go     # Custom error types
│   ├── panic_recover.go    # panic and recover
│   └── error_wrap.go       # Error wrapping
│
├── 09_stdlib/              # Standard library coverage
│   ├── fmt/
│   │   ├── print.go        # Print, Printf, Println
│   │   ├── sprint.go       # Sprint, Sprintf, Sprintln
│   │   ├── fprint.go       # Fprint, Fprintf, Fprintln
│   │   └── scan.go         # Scan, Scanf, Scanln
│   │
│   ├── strings/
│   │   ├── basic_ops.go    # Contains, HasPrefix, etc.
│   │   ├── builder.go      # strings.Builder
│   │   └── split_join.go   # Split, Join operations
│   │
│   ├── io/
│   │   ├── reader_writer.go # io.Reader, io.Writer
│   │   ├── copy.go         # io.Copy
│   │   └── ioutil.go       # ioutil functions
│   │
│   ├── os/
│   │   ├── file_ops.go     # File operations
│   │   ├── env.go          # Environment variables
│   │   └── args.go         # Command line args
│   │
│   └── time/
│       ├── now.go          # time.Now()
│       ├── sleep.go        # time.Sleep()
│       └── duration.go     # time.Duration
│
├── 10_keywords/            # Ensure all Go keywords are tested
│   ├── keywords_test.go    # Comprehensive keyword coverage
│   └── README.md           # Checklist of all Go keywords
│
├── 11_edge_cases/          # Tricky cases
│   ├── shadowing.go        # Variable shadowing
│   ├── init_func.go        # init() functions
│   ├── blank_identifier.go # _ usage
│   ├── iota.go            # iota in const
│   └── type_alias.go      # type aliases
│
├── 12_integration/         # Real-world patterns
│   ├── http_server.go      # Simple HTTP server
│   ├── json_parse.go       # JSON encoding/decoding
│   ├── file_reader.go      # File reading program
│   └── cli_tool.go         # Command-line tool
│
└── 99_bootstrap/           # The ultimate test
    └── go2rust/            # Copy of go2rust source
        ├── main.go
        ├── transpile.go
```

## Test Naming Conventions

1. **Numbered directories** for phases (01_basics, 02_types, etc.)
2. **Descriptive names** that match Go feature names
3. **Grouped by concept** not by implementation difficulty
4. **Keyword/function coverage** tracked explicitly

## Test Output Preservation

**IMPORTANT**: The test system preserves transpiled output files as snapshots:

- **`.rs` files** - The transpiled Rust code
- **`Cargo.toml` files** - Generated Cargo manifests
- **`Cargo.lock` files** - Dependency lock files

These files serve as:

1. **Test output artifacts** - Show exactly what the transpiler produced
2. **Regression detection** - Git tracks changes to transpiler output over time

**Only build artifacts should be cleaned**:

- `target/` directories (Rust build output)
- `debug/` directories (Debug build output)
- Go binaries in XFAIL tests (from validation)

The `teardown()` function in `tests.bats` must NEVER delete .rs, Cargo.toml, or Cargo.lock files.

## Coverage Tracking

Create coverage files to ensure completeness:

```
tests/coverage/
├── keywords.md         # Checklist of all Go keywords
├── operators.md        # All operators (+, -, *, /, %, etc.)
├── builtins.md        # All builtin functions (make, len, cap, etc.)
├── stdlib_fmt.md      # All fmt package functions
├── stdlib_strings.md  # All strings package functions
└── ...
```

## Test File Template

Each test should have a header comment explaining what it tests:

```go
// Test: Variable declarations with type inference
// Features: var keyword, := operator, basic types
// Expected: Correct type inference and initialization

package main

func main() {
    var x int = 42
    y := "hello"
    z := 3.14
    println(x, y, z)
}
```

## Benefits of This Organization

1. **Easy to find**: Need to test `defer`? Look in 04_functions/defer.go
2. **Progressive**: Tests build on each other through phases
3. **Complete**: Explicit coverage tracking ensures nothing is missed
4. **Discoverable**: New contributors can easily add tests
5. **Scalable**: Structure handles growth from 10 to 1000+ tests

## Implementation Plan

1. Start with current tests in 01_basics/
2. Add 02_types/ next (Phase 2 from README)
3. Create coverage tracking files
4. Gradually fill in each category
5. Use TODO comments for unimplemented features

## Test Utilities

Consider adding helper scripts:

```
tests/
├── utils/
│   ├── generate_test.sh    # Create new test from template
│   ├── check_coverage.sh   # Report coverage gaps
│   └── validate_names.sh   # Ensure naming conventions
```
