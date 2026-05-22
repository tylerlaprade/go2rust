# Wrapper and Raw Value Rules

Use this when generated Rust fails with missing `.borrow()` / `.lock()` methods, nested wrappers, moved handles, aliasing breaks, or wrong Rust value categories.

## Core Boundary

- Treat wrapper/raw decisions as type-boundary decisions, not syntax decisions. Use `TypeInfo.ReturnsWrappedValue`, `TypeInfo.NeedsUnwrapping`, and the exact caller context.
- Type conversions already emit wrapped results when their Go result is wrapped. Returns, assignments, and arguments must not wrap them again.
- Raw producers such as literals, binary expressions, indexing, `len`, `cap`, `min`, `max`, numeric conversions, and slice bounds should be cast or cloned as raw Rust values before any expected-type wrapper is applied.
- A generated `.borrow()` or `.lock()` call on a primitive, literal, string, integer, `usize`, or stdlib interface stub usually means the caller confused a raw value for a wrapper.
- Assignment into a wrapped target must evaluate the RHS first, then open the LHS mutable borrow. This preserves Go evaluation order and avoids nested `RefCell` / `Mutex` failures.
- Rust `Clone` on generated structs stays shallow so handle-shaped fields preserve aliasing. At explicit Go value-copy boundaries, use `__go_value_clone()` so scalar, array, and nested struct fields get fresh wrappers.
- Selector RValue lowering for scalar fields already clones the inner value. Use selector LValue context only when the caller needs the field handle itself.
- Syntax registries may refine clone/handle details only after `go/types` has already proven the operation and boundary.

## Maps

- Map keys are raw comparable Rust values, except pointer keys, which use the pointer-key helpers. Do not use wrapper handles as map keys.
- Map-valued selector fields are wrapped handles. Reads, comma-ok reads, ranges, writes, and `delete` must clone/borrow the field handle before touching the inner `BTreeMap`.
- Short declarations from selector map fields copy the map handle. Assigning a map field replaces that handle. Package-global maps use the package-global path instead.
- Short declarations and inferred locals from package-global maps snapshot the current inner `BTreeMap` into a new wrapper. The same rule applies to package-global slices.
- Wrapped map range sources must clone the inner map inside a block and drop the borrow guard before the loop body.
- Owned non-copy map range keys must be cloned before insertion, wrapping, or call arguments if the Go variable is reused later.
- Wrapped map range values are handles. Clone or unwrap the inner value according to the map element type; do not move from the ranged wrapper.
- Map values whose Go type is handle-shaped (`*T`, `[]T`, `map[K]V`, `chan T`, `func`, `interface`, or `error`) must preserve the stored handle through lookup, comma-ok, short declarations, calls, returns, and assignments.
- Nil map values for handle-shaped element types are nil handles, not `Some(None)`.
- Pointer and interface map values are handles. Do not deep-copy pointees or interface payloads unless Go value semantics require a real copy and the inner type is cloneable.
- Assignments into pointer-valued maps must preserve the RHS pointer handle, including selector-field RHS values.
- Struct map keys require package-wide Eq/Ord discovery, not same-file scanning. Concurrent ordering must clone field values out before comparing.
- If a non-copy bare key is also used by the assignment RHS, clone the key for insertion before evaluating the value.
- For map lookups used as range sources, inspect the map value type. Handle-shaped values return handles that must be borrowed before iteration; scalar/struct values may iterate the cloned bare value.

## Stdlib Interface Keys And Values

- Stdlib-interface map keys store raw comparable stubs. Convert concrete keys to the go/types-proven interface stub before pointer-key lowering.
- Range over a stdlib-interface-keyed map exposes a wrapped interface handle to Go code, but insertions/lookups still unwrap and clone the raw stub key.
- Passing a stdlib-interface map range key to the same interface parameter should pass the existing wrapped handle. Do not wrap the handle again.
- Bare stdlib-interface values from slice range/index paths are raw stubs, not handles. Nil checks and single-value type assertions should use that representation directly.
- Stdlib named-interface slice literals need explicit Rust element types when non-empty, for example `Vec::<types_Type>::from([...])`.
- Selector fields passed to a different stdlib named interface must use the selector handle in LValue context before conversion.

## Pointers And Package Globals

- Pointer assignment is handle assignment. `p = q`, pointer fields, package pointer vars, and pointer composite literals must preserve pointer identity by replacing handles.
- Pointer equality is handle identity with nil handling, not pointee equality.
- Address-of expressions already return pointer handles. Short declarations and call arguments must not wrap them again when go/types proves a pointer type.
- Inside pointer-receiver methods, `self` is already `&mut T` or `&T`. `*p = T{...}` should assign through `*self`, not through wrapper slots.
- Pointer fields inside wrapped structs require mutable borrow of the outer struct when replacing the field handle. Scalar field assignment can mutate the field wrapper through an immutable outer borrow.
- Increment, decrement, and compound assignment through nested field handles should clone the target handle before opening the mutable borrow.
- Package-level pointer globals have two layers: the package-global slot and the Go pointer handle stored inside it. Reads, writes, nil comparisons, direct dereferences, returns, and field selectors must unwrap the right layer.
- Package pointer globals initialized from constructors should store the returned pointer handle, not the pointee.
- Stdlib package pointer variables in generated stubs are accessor functions returning pointer handles. External transpiled package globals still need the package-global slot unwrapped before method calls.
- Promoted embedded pointer fields must preserve the promoted field path through named variables and indexed pointer expressions.

## Interfaces And Errors

- `interface{}` / `any` values are handles to `Box<dyn Any>`. Passing or assigning an existing `any` clones the handle; do not re-box from a shared reference.
- The predeclared `any` alias follows the same path as explicit `interface{}`.
- The predeclared `error` is special. Slices and maps of `error` store normal wrapped error handles; `chan error` carries the nullable payload option.
- Sending on `chan error` moves the payload with `take()`. Receiving into an `error` variable stores that payload directly; select receives wrap the payload for the case body.
- Returning or assigning selector fields of type `error` clones the error handle, not the boxed payload.
- Concrete values that implement `error` must be boxed when assigned, returned, or passed as `error`; prove this with `types.Implements`.
- Do not compare `Box<dyn Error>` values directly. Lower `error == error` through wrapper state until a real comparable error identity model exists.
- A struct embedding `error` implements `error` through promoted `Error() string`; generate the promoted method and manual `Debug` when trait fields block derive.

## Slices, Arrays, Ranges, And Builtins

- `usize` contexts such as indexes, bounds, capacities, and `make([]T, len, cap)` need raw integers. Wrapped Go integer results must be unwrapped before `as usize`.
- `len` and `cap` emit Rust `usize`. When the expected Go type is `int`, cast to `i32` inside the target wrapper.
- Short declarations and `var` declarations from `len` / `cap` should initialize normal wrapped Go `int` handles, not bare `usize` locals.
- Range indexes are bare `usize`. Cast at Go `int` use boundaries; do not change the range variable itself into a wrapper.
- Range over a Go string yields byte index plus rune value. Keep char comparisons as Rust char literals, and cast range chars to `i32` only when go/types expects `rune` / `int32`.
- Range values from `[]string` are iterator references. Clone `(*value).clone()` when an owned string is expected, used in a literal, appended, compared, or passed to a string helper.
- Reassigned range variables over slices or strings need owned mutable Rust bindings.
- Range over wrapped slice-returning calls and pointer-to-array expressions should use borrowed slice views.
- Array/slice elements whose element type is a pointer or channel must preserve existing handles in literals and append paths.
- Nested slices are bare `Vec<T>` inside the outer collection. Mutate inner vectors directly for `arr[i] = append(arr[i], x)` and `arr[i][j] = value`.
- `append` should evaluate its target once, bind the handle, mutate it, and return the same handle.
- Variadic `append(dst, src...)` extends from unwrapped source slice values.
- `copy` must treat slice-expression destinations as mutable views into their backing storage and unwrap source values before copying.
- `make([]T, n)` declarations may need explicit Rust types when zero values are ambiguous, especially `Default::default()` and `error` handles.
- `slices.Clone` and `slices.Clip` must preserve the go/types-proven element type. Do not route them through generic stubs that return `[]any`.

## Constants, Strings, And Named Types

- Package constants and package globals are separate facts. Type-info object identity wins, then the package-constant registry.
- Local constants resolve by go/types object identity, not identifier text.
- String constants are bare Rust values. Use them directly for indexing/slicing, and add `.to_string()` only when the expected value is owned `string`.
- Package string constants used as `map[string]...` keys need owned-string treatment for insertion and lookup.
- Byte-like constants use `go/constant` plus expected go/types context. Do not infer byte/rune handling from names.
- Local untyped constants assigned into byte-sized fields or vars need the expected type cast.
- Go 1.21 `min` / `max` are predeclared builtins only when go/types resolves the callee to `types.Builtin`; shadowed identifiers use the normal function path.
- Named scalar map values must preserve the named type in go/types-driven Rust type strings.
- Named scalar receivers should use `self.0`; named slice methods pass the named slice handle with `self.clone()`.
- Named numeric type definitions need operator/comparison impls tied to the actual numeric underlying type.
- Named integer constants, conversions, bitwise expressions, and comparisons should use go/types to construct or unwrap the correct named newtype. Do not depend on file-order-sensitive lookup helpers.
- External stdlib named integers are bare tuple structs. Const declarations for them should emit folded constant values directly.
- `time.Duration` lowers through `std::time::Duration` helpers, not a tuple newtype.
- External named string types are dependency newtypes. Construct the dependency value and compare through its inner string where needed.

## Strings And Formatting Helpers

- `strings.Builder` lowers to the Rust `String` helper shape. Recognize both value and pointer receivers before generic method lowering.
- Short-declared `strings.Builder{}` values are bare `String`s; their methods emit direct `push_str`, `push`, `clone`, and `len` calls.
- Wrapped builders must clone through a local guard block and drop the guard before returning the wrapped string.
- Nested `strings` helper calls should materialize owned-string arguments before calling Rust `String` / `str` methods.
- Dynamic concatenation emits raw `String`. Pass it to helper calls as owned or borrowed raw string as the expected type requires.
- Stdlib string helpers should treat go/types-proven string constants as bare arguments.
- `[]string` index expressions already produce bare owned `String` values in RValue context.
- Format arguments that are selector expressions with interface or slice type often need selector LValue context or `format_slice_values(&...)` to avoid adding wrapper operations to raw values.
- Formatting `[]*T` needs pointer-element-specific helpers when go/types proves element `String() string` methods.

## Package And Module Boundaries

- Register package-wide type facts before per-file emission in every package path. Cross-file aliases, interfaces, function signatures, named non-struct definitions, and method sets cannot depend on file emission order.
- Generated helper types with Rust identity, such as `GoTime`, `GoContext`, and `GoChannel`, must be package-scoped for multi-file crates.
- Helper includes inserted into the crate root must de-duplicate shared imports.
- Non-main module init aggregators need module-specific Rust names so sibling glob imports do not collide.
- Shared external helper types that cross dependency crate boundaries belong in `vendor/go2rust_stdlib_stubs`, not local file helpers.
- Package selector and package-call detection must resolve from the selector base object's `*types.PkgName`, with import-name fallback only when type info has no object for that identifier.
- Go names that collide with Rust prelude names must use escaped Rust names consistently in declarations, constructors, impls, literals, locals, and imports.
