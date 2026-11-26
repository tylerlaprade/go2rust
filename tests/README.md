# Test Suite

This directory contains all transpiler tests. Each directory containing a `main.go` file is a test case.

## Test Structure

### Working tests

These are tests for features that currently work:

```
tests/
└── hello_world/
    ├── main.go          # Go source
    └── main.rs          # Generated Rust (tracked in Git)
```

### XFAIL tests (expected failures)

These are tests for features not yet implemented:

```
tests/
└── XFAIL/
    ├── variable_declarations/
    │   └── main.go      # Go code using var/short declarations
    ├── pointers_basic/
    │   └── main.go      # Go code using pointers and structs
    └── methods/
        └── main.go      # Go code with method definitions
```

## How It Works

1. Each directory in `tests/` containing a `main.go` is a test case
2. The test transpiles all `.go` files in the directory
3. Generated `.rs` files are tracked in Git as examples
4. Runs both versions and compares outputs - they must match exactly

## XFAIL Auto-Promotion

XFAIL tests automatically promote to working tests when they start passing:

1. **Auto-detection**: `./test.sh` checks if XFAIL tests now transpile successfully
2. **Auto-promotion**: Passing XFAIL tests are moved from `tests/XFAIL/` to `tests/`
3. **Git diff catches changes**: CI will detect the moved files and require manual review
4. **Clear feedback**: You'll see "🎉 Promoting XFAIL test 'feature_name'" messages

This ensures new features are properly reviewed before being considered "working".

## Adding New Tests

### Working test

```sh
mkdir tests/my_feature
echo 'package main...' > tests/my_feature/main.go
# Test will be auto-discovered - no manual steps needed!
```

### XFAIL test (planned feature)

```sh
mkdir tests/XFAIL/my_future_feature
echo 'package main...' > tests/XFAIL/my_future_feature/main.go
# Test will show as "skip" until the feature is implemented
# When implemented, it will auto-promote to tests/
```

### Multi-file tests

For tests that require multiple Go files (e.g., testing function calls across files):

```sh
mkdir tests/my_feature
echo 'package main...' > tests/my_feature/lib.go   # Library functions
echo 'package main...' > tests/my_feature/main.go  # Main function that calls lib
# Test will be auto-discovered - no manual steps needed!
```

The test framework will:

1. Run `go run .` in the directory to get expected output
2. Transpile all `.go` files to Rust modules
3. Generate appropriate `main.rs` with module declarations
4. Create a `Cargo.toml` for the project
5. Run the Rust version and compare outputs

Example: `library_example` test demonstrates:

- Multiple Go files in the same package
- Function calls between files
- Proper module organization in Rust
- Automatic handling of `lib.go` → `lib_.rs` renaming to avoid Rust naming conflicts

## Running Tests

```sh
# Install BATS (one time)
brew install bats-core  # macOS
# or: npm install -g bats

# Run all tests (auto-discovers new tests)
./test.sh

# Run specific test
./test.sh
bats tests.bats --filter "hello_world"
```

### A program that does not exit 0

A program that ends with a panic nothing recovers exits with status 2, and `go run` reports that as a failure. Name the status in the fixture's `.go2rust.toml`:

```toml
exit_status = "2"
```

The harness then builds the Go program and runs it directly, and requires both programs to end with that status. The output is compared up to Go's goroutine trace: the first `goroutine N [...]` line and everything after it, and a `[signal ...]` line, are left out, because the translation cannot reproduce the trace. `expected_output.txt` holds the output without them.
