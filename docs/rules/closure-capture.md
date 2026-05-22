# Closure Capture Rules

Use this when generated Rust moves a value into a closure too early, captures a non-local that should not be captured, misses a real captured variable, or shares outer defer state with a nested function literal.

## Capture Analysis

- Closure capture decisions are object-based through `go/types`, not name-based.
- Package selectors, type names, fields, and variables declared inside the function literal are not outer locals.
- In keyed composite literals inside closures, skip `KeyValueExpr.Key` only when go/types proves the key is a struct field with `types.Var.IsField()`. Map literal keys may still be captured variables.
- Type switch case variables declared by `switch v := x.(type)` live in `types.Info.Implicits` on each `*ast.CaseClause`. Treat them as local to the function literal.
- Nested function literal bodies should not cause pre-clones in the outer statement. Analyze each literal at its own scope.
- Always escape captured Rust identifiers with the existing Rust identifier helpers.
- Apply `currentCaptureRenames` anywhere a captured identifier can appear, including map assignment/update/lookup paths, external stub calls, package-function calls, and selector-method arguments.

## Range Captures

- Captured range variables over `[]string` are owned closure clones. After `filename_closure_clone := filename.clone()`, string argument lowering should use `filename_closure_clone.clone()`, not `(*filename_closure_clone).clone()`.
- Captured map targets must use the renamed handle inside move closures. For `visit = func(...) { view[id] = ... }`, use `view_closure_clone`, not the consumed outer binding.
- External stdlib stub calls inside closures must apply capture renames before cloning wrapped identifiers.
- Method-call arguments that are function literals and capture the receiver need a receiver-handle clone before the receiver is borrowed for the method call.
- If a closure captures a wrapped range variable used by `sort.Slice` or another external stub, pass the range handle clone before owned-expression lowering.

## Function Literals And Defers

- Function literals own their defer state. Do not let nested or deferred function literals inherit outer `currentFunctionHasDefer`.
- A `return` inside `defer func(){...}` returns from that closure, not the enclosing function.
- Function literals with named result parameters need local result slot declarations, just like named functions.
- Function and method emitters both need the same final-defer logic. Emit a trailing defer-drain block only when the final statement can fall through.
- The final statement is terminating when it is a `return`, terminating `switch`, terminating type switch, or panic; an extra trailing block can break Rust's final-expression type.
- Defer detection must scan select, switch, type-switch, and labeled statement bodies without entering nested function literals.
- A `defer` inside a select communication case belongs to the enclosing function.
- Explicit returns from named-result functions with defers must use declared result types for synthetic result assignments when go/types has no entry for the synthetic identifier.

## Function Values

- Function literals passed as immediate `sync.Once.Do(func(){...})` callbacks should not use ordinary statement-level pre-clone handling; the callback is invoked synchronously by the helper.
- Go function values are mutable closures. Use `FnMut`, not `Fn`.
- Function-typed values are handles. Appending, indexing, assigning, passing, and calling them should preserve/clone the handle rather than unwrapping to the boxed closure.
- Struct `Display` for `[]func` fields should use an opaque function placeholder per element.
- Struct `Display` should delegate to a go/types-proven `String() string` method before field formatting, checking named and pointer method sets.

## Control Flow In Captured Contexts

- Select communication cases add a synthetic Rust `break` only when the case body can fall through.
- If a select case returns or panics, a trailing `break;` can force the surrounding loop expression toward `()` and break functions whose final select returns a value.
- A `for` init short declaration can shadow an active outer range variable. Emit the initializer with the outer range visible to the RHS, hide it for the inner loop condition/body/post, wrap the shadowing loop in a Rust block, and restore the outer range afterward.
- In `for init; cond; post` loops, Go runs the post statement before an unlabeled `continue`, even when the continue is nested inside a switch/type switch. Track nearest loop posts separately from labeled posts.
