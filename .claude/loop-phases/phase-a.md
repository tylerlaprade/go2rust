# Phase A: Easy/Medium XFAIL Tests

Pick off XFAIL tests that need small, scoped transpiler fixes.

## Per-Session Workflow

1. Build: `go build -o go2rust ./go`
2. Run `./test.sh 2>&1 | tail -20` to see current counts
3. Survey XFAIL tests to find the easiest target:
   - For each candidate, run `./go2rust tests/XFAIL/<name>/main.go 2>&1 | head -80` and compare with `tests/XFAIL/<name>/expected_output.txt`
   - Pick the test with the smallest gap between current output and expected
4. Read the test's `main.go` to understand what Go features it uses
5. Identify what the transpiler gets wrong — read the relevant `go/*.go` source
6. Make the minimal fix
7. Build: `go build -o go2rust ./go`
8. Test: `./test.sh <name>` — if it passes, auto-promotion moves it from XFAIL
9. Run FULL suite: `./test.sh 2>&1 | tail -5`
10. **REGRESSION CHECK**: If any previously-passing test now fails, `git checkout -- go/` immediately and try a different approach
11. Stage and commit: `git add -A && git commit -m "<description>"`
12. Repeat from step 3

## Target Tests (roughly in priority order)

Easy language features:
- recursion_basic, variadic_functions, multiple_returns
- switch_statements, range_loops, range_iteration
- pointers_basic, pointers_dereference, slices_append

Type system:
- type_conversions, type_assertions, type_assertion_simple
- named_types_methods, function_types

Structs:
- structs_basic, anonymous_structs_basic, anonymous_structs_functions
- embedded_structs, embedded_method_promotion, nested_structures, nested_embedding

Constants:
- const_expressions, const_iota, const_string_concat, const_typed, enums_iota

Strings/slices:
- strings_runes, string_builder, slice_operations_advanced

Error handling:
- error_handling, errors_custom

Control flow:
- fallthrough_switch, labeled_statements, blank_identifier

## Skip These (other phases handle them)

- Anything with channels, goroutines, select, waitgroup, mutex, atomic
- cross_file_*, init_functions, init_order_complex, stdlib_imports
- json_marshal, file_io, http_client, regex_basic, crypto_hash, etc.
- generics_basic, struct_tags_reflection, unsafe_pointer_ops

## Rules

- One test at a time. Fix, verify, commit.
- If a fix needs >150 lines of transpiler changes, commit partial progress and move on.
- If stuck for more than 3 attempts on one test, skip it and try another.
- Never break existing tests. The regression check is non-negotiable.
- Commit after every successful test promotion.
