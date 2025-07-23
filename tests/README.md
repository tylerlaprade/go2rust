# Test Suite

This directory contains all transpiler tests. Each `.go` file is a test case.

## Test Structure

### Simple tests (no stdin)

```
tests/
├── hello_world.go        # Test file
└── hello_world.rs        # Generated output (git-ignored)
```

### Tests with stdin inputs

```
tests/
├── echo_program.go       # Test file
├── echo_program/         # Input directory (same name as .go file)
│   ├── test1.txt        # Input case 1
│   └── test2.txt        # Input case 2
└── echo_program.rs       # Generated output (git-ignored)
```

## How It Works

1. Each `.go` file in `tests/` is a test case
2. The test transpiles `foo.go` to `foo.rs`
3. If directory `foo/` exists, runs both versions with each file as stdin
4. Otherwise, runs both versions without stdin
5. Compares outputs - they must match exactly

## Adding New Tests

### Simple test (no inputs)

```bash
echo 'package main...' > tests/my_feature.go
# Test will be auto-discovered - no manual steps needed!
```

### Test with inputs

```bash
echo 'package main...' > tests/my_feature.go
mkdir tests/my_feature
echo 'test input' > tests/my_feature/case1.txt
# Test will be auto-discovered - no manual steps needed!
```

### Multi-file tests

For tests that require multiple Go files (e.g., testing function calls across files):

```bash
mkdir tests/my_feature
echo 'package main...' > tests/my_feature/lib.go   # Library functions
echo 'package main...' > tests/my_feature/test.go  # Main function that calls lib
# Test will be auto-discovered - no manual steps needed!
```

The test framework will:
1. Run `go run lib.go test.go` to get expected output
2. Transpile both files to Rust
3. Concatenate them (temporary solution until proper module support)
4. Run the Rust version and compare outputs

## Running Tests

```bash
# Install BATS (one time)
brew install bats-core  # macOS
# or: npm install -g bats

# Run all tests (auto-discovers new tests)
./test.sh

# Run specific test
./test.sh
bats tests.bats --filter "hello_world"
```
