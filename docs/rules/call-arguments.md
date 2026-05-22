# Call Argument Rules

Use this when a function or method call emits nested handles, moves out of shared references, loses interface conversions, gets variadic arity wrong, or borrows a receiver before evaluating its arguments.

## General Argument Boundary

- Most function and method parameters expect wrapped handles. Caller helpers often open a wrapper before delegating; do not wrap an expression again if it already produced the parameter's handle shape.
- Go parameters can be rebound. Emit `mut` on a Rust parameter only when the function body assigns to or increments/decrements that parameter.
- If an argument helper runs inside an already-open wrapper, emit the raw inner value for that context.
- Function literals passed as arguments should use the boxed function-literal path, not generic expression lowering, because generic expression lowering wraps the closure itself.
- Slice, map, and array composite literals passed to package functions or methods should pass the literal handle directly. Wrapping again creates `Option<Handle<...>>` where the callee expects `Option<...>`.
- `nil` passed to a package function or method must use the typed nil handle before the generic wrapper path.
- Return expressions that already produce pointer handles should pass through only when the declared return type is pointer-typed. Interface returns still need the normal interface conversion.
- Explicit returns from named-result functions with defers use a synthetic assignment before draining defers. Use the declared result type when the synthetic identifier lacks a go/types entry.
- Blank named result slots have no assignable variable; skip synthetic result assignment for those slots.

## Pointer Arguments

- Pointer-producing expressions passed to `*T` parameters are already pointer handles. This includes address-of locals, selectors, composite literals, indexed `[]*T` values, and pointer type assertions.
- If generated Rust contains `Some(handle.clone())` where the parameter expects `Arc/Rc<...Option<T>>`, the call path nested a pointer handle.
- Address-of selector arguments such as `&r.field` should clone the field handle when go/types proves assignability to `*T`.
- Address-of slice elements must use the same index lowering as ordinary slice access. Route named integer indexes through `writeExpressionAsUsize`.
- Indexed `[]*T` values passed to `*T` parameters should pass the indexed pointer handle directly.
- Fixed arguments in variadic calls still need pointer-special handling before wrapping.
- When a pointer receiver initializes a struct pointer field with itself, use go/types assignability and rewrap the receiver handle. Bare `self.clone()` is the struct value, not the Go pointer handle.

## Interface Arguments

- Named stdlib interfaces are wrapped at function boundaries. Passing one wrapped interface into another wrapped interface requires unwrapping/cloning the raw stub before the caller's wrapper is applied.
- Imported transpiled interfaces are Rust traits in dependency crates. Prove current-package concrete-to-imported-interface relationships with `go/types.Implements`, then emit the dependency trait impl beside the concrete type.
- Imported interface impls can be required by calls in files other than the concrete type declaration. Collect them at package scope before per-file module emission.
- Local concrete values implementing stdlib named interfaces should construct the target stub value directly when returned or stored. Do not register a source conversion in the shared stdlib stub crate for local source types.
- Selector fields passed to interface parameters should use selector LValue context when RValue lowering would unwrap to a raw stub too early.
- Interface equality with a concrete pointer operand should dispatch through the interface trait helper, not raw Rust `==` on boxed trait values.
- Function-local interface declarations used in type assertions or type switches should be hoisted into emitted Rust traits. Preserve Go interface identity from go/types and emit concrete assertion arms from actual `types.Implements` relationships.
- Typed constants passed to local interface parameters should construct the go/types-proven named concrete type and pass a trait-object reference.

## Function Values

- Named Go function types are aliases to wrapped closure handles. Track function-type aliases separately from general aliases.
- Type conversions between named function types should preserve the wrapped function handle unless the callee explicitly expects a bare closure box.
- Methods on named function types need an extension trait and trait impl for the alias. Rust aliases cannot have inherent impls.
- Function type signatures must use return-type lowering for results. Pointer returns are pointer handles, not wrappers around pointer handles.
- Function-typed named results are nil handles by default. For explicit returns with defers, assign the boxed function into the named result slot rather than wrapping the whole function handle again.
- Go function values lower to `Box<dyn FnMut...>`, not `Box<dyn Fn...>`.
- Function-typed parameters and locals are already handles. Passing one to another `func` parameter clones the handle; do not unwrap to the boxed closure, which is not cloneable.
- Pointer-receiver method values must evaluate the receiver once, clone the pointer handle into the boxed closure, and call the method inside the closure.
- Package function selectors assigned into function-typed fields need a boxed closure handle. Function-typed selector values copied from another field should clone the function handle.
- Named function declarations stored in function-typed map values must be boxed through the normal function-value handle path; raw Rust fn items have distinct types.
- Function-value calls must open the wrapped function slot mutably, derive a typed raw pointer from go/types, drop the wrapper guard, then invoke `FnMut`.
- When deriving a raw call pointer type, check known stdlib named helper types before falling back to the signature. `context.CancelFunc` is `GoCancelFunc`.
- Function type assertions that immediately call the asserted function need mutable interface borrow and `downcast_mut`.
- Function-field selector fallback must be receiver-scoped and type-proven. Do not use global unique-field fallback when the receiver type is known.

## Variadics

- Register function signatures for every file before emitting modules; cross-file variadic helpers need signature facts before call lowering.
- Variadic function and method calls pack trailing operands into one wrapped vector argument. Do not emit each variadic operand as a separate Rust parameter.
- Variadic function-value calls also need go/types signature packing, including an empty variadic vector when no trailing operands are provided.
- Package selector calls into transpiled dependencies need the same variadic packing and stdlib-interface argument conversion as local calls.
- External stdlib variadic stubs have one generated generic parameter for the variadic bundle. Calls such as `io.MultiWriter(a, b)` lower to one tuple-shaped variadic argument.
- `...any` elements need interface boxing.

## External Stubs And Stdlib Helpers

- External stdlib stubs consume generic arguments by value. Clone non-copy range variables at the call site if the Go variable is reused later.
- External stdlib stubs are generic at the Rust boundary, but Go pointer arguments still need handle semantics. Pass pointer expressions as LValue handle clones when go/types proves that shape.
- External stdlib stub argument paths bypass much of local call-argument lowering. Apply `currentCaptureRenames` before cloning captured wrapped identifiers inside closures.
- Wrapped range variables passed to external stdlib stubs should pass the range handle clone before falling back to owned-expression lowering.
- `os.File` has local helper and external stub representations. Keep local `os.Create` / `GoFile` paths out of external-stub argument mode while still registering direct external `os_File` methods where needed.
- `json.Marshal` struct support should stay go/types-driven, including deterministic map-key order from `BTreeMap`.
- Stdlib string helpers returning wrapped strings should first materialize owned string arguments before calling Rust string methods.
- The predeclared `error` type should receive wrapped error handles at function and method boundaries, not generic named-interface trait references.
- `context.Context.Err` returns `error`, not `string`; helper storage and returns must use wrapped boxed error handles.
- `GoContext` needs stable `Display` and `Debug` implementations because it appears in generated structs and format calls.

## Receivers And Methods

- Method Rust-name disambiguation is receiver-scoped and selector-driven by go/types method identity.
- Receiver mutability analysis must use the package-wide method set, not only methods declared in the current file.
- Promoted method forwarders must preserve the embedded method's receiver mutability. Read-only promoted pointer methods use `&self`; mutating ones use `&mut self`.
- Embedded external stdlib and imported transpiled types need promoted forwarding methods generated from go/types method sets. Emit forwarding impls only in the module declaring the embedding type.
- For pointer-receiver method calls on the current receiver, evaluate arguments that read or call through that receiver into locals before emitting `self.method(...)`.
- If a method-call argument is a function literal that captures the receiver variable, clone the receiver handle into a local before borrowing it for the call.
- Method calls through selector fields must align the field borrow kind with the receiver unwrap. Pointer receivers using `.as_mut().unwrap()` need mutable borrow/lock.
- Channel sends that depend on method result types need go/types method signatures. If missing, emit the normal type-information-required error.

## Package Init And Names

- Package init aggregators should call only real Go `init` functions. Do not derive init calls from all function-name overrides.
- Package-wide type facts must be registered before per-file emission in `ProjectGenerator`, `UnifiedTranspiler`, and `PackageLoader`.
- Package function selectors assigned to fields, init aggregators, and method disambiguation all need go/types identity, not name-only matching.
