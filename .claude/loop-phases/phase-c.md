# Phase C: Multi-File Package Support

The transpiler currently handles single-file packages. This phase adds support for multi-file packages, init functions, and cross-file references.

## Target Tests (~8)

cross_file_func_vars, cross_file_maps, cross_file_methods, cross_file_types,
init_functions, init_order_complex, stdlib_imports, blank_imports_side_effects

## Session 1: Understand the Problem

1. Read all target test directories — they have multiple `.go` files
2. Read `go/transpile.go` to see how file input currently works
3. Understand what breaks: types defined in one file referenced in another, functions called across files, init() ordering
4. Design the approach:
   - Accept directory input (already partially supported?)
   - Collect declarations from ALL files before generating code
   - Generate a single `main.rs` (or `lib.rs` + `main.rs` with modules)
   - Handle init() functions: generate init calls at start of main() in correct order
5. Write design to `go/MULTIFILE_DESIGN.md`, commit

## Session 2-3: Implementation

1. **Multi-file collection**: Modify `TranspileWithMapping()` to accept multiple files, collect all declarations
2. **Cross-file types**: Ensure struct/interface declarations from all files are visible
3. **Cross-file functions**: All functions from all files go into the same output
4. **Init functions**: Collect all `init()` functions, generate calls at top of `main()` in file-order then alphabetical (Go spec)
5. Test with `cross_file_types` first (simplest), then expand

## Regression Protocol

- Multi-file support must not change behavior for single-file input
- Run full `./test.sh` after every change
- If regressions: `git checkout -- go/` immediately
