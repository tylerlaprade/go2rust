# Test Suite

This directory contains all transpiler tests. Each directory containing a `main.go` file is a test case.

For the comprehensive test organization strategy and future test planning, see [TEST_ORGANIZATION.md](TEST_ORGANIZATION.md).

## Test Structure

### Simple tests (no stdin)

```
tests/
└── hello_world/
    └── main.go          # Test file
```

### Tests with stdin inputs

```
tests/
└── echo_program/
    ├── main.go          # Test file
    └── inputs/          # Input directory
        ├── test1.txt    # Input case 1
        └── test2.txt    # Input case 2
```

## How It Works

1. Each directory in `tests/` containing a `main.go` is a test case
2. The test transpiles all `.go` files in the directory
3. If directory `inputs/` exists within the test, runs both versions with each file as stdin
4. Otherwise, runs both versions without stdin
5. Compares outputs - they must match exactly

## Adding New Tests

### Simple test (no inputs)

```bash
mkdir tests/my_feature
echo 'package main...' > tests/my_feature/main.go
# Test will be auto-discovered - no manual steps needed!
```

### Test with inputs

```bash
mkdir tests/my_feature
echo 'package main...' > tests/my_feature/main.go
mkdir tests/my_feature/inputs
echo 'test input' > tests/my_feature/inputs/case1.txt
# Test will be auto-discovered - no manual steps needed!
```

### Multi-file tests

For tests that require multiple Go files (e.g., testing function calls across files):

```bash
mkdir tests/my_feature
echo 'package main...' > tests/my_feature/lib.go   # Library functions
echo 'package main...' > tests/my_feature/main.go  # Main function that calls lib
# Test will be auto-discovered - no manual steps needed!
```

The test framework will:
1. Run `go run .` in the directory to get expected output
2. Transpile all `.go` files to Rust
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

## Test Organization

See [TEST_ORGANIZATION.md](TEST_ORGANIZATION.md) for:
- Comprehensive test categorization strategy
- Coverage tracking approach
- Naming conventions
- Future test planning
