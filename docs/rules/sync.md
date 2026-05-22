# Sync And Evaluation Rules

Use this when generated Rust holds locks too long, clones the wrong synchronization helper, deadlocks in map/range paths, or starts a mutable receiver borrow before evaluating receiver-dependent arguments.

## `sync.Mutex`

- `sync.Mutex` fields are bare helper values, but cloning the helper must clone the same underlying lock. A fresh lock breaks Go semantics.
- For `mu.Lock(); defer mu.Unlock()`, emit a local cloned mutex handle and lock that local. Locking `self.mu` directly can hold an immutable `self` borrow for the guard lifetime and block later `&mut self` calls.
- Suppressing `defer mu.Unlock()` is correct only when the generated RAII guard stays alive for the intended Go scope.
- A direct `sync.Mutex.Unlock()` after a tracked `Lock()` should drop the generated guard for that receiver.
- `if` / `else` branches need independent active-guard snapshots. After the branch, keep only guards that remain active on every path.
- Package-global map snapshots such as active mutex guard maps follow package-global snapshot semantics before mutation.

## `sync.Once`

- `sync.Once.Do(func(){...})` should inline the callback into the method call and clone only the `GoOnce` handle before invoking it.
- Do not route immediate `Once.Do` callbacks through the ordinary boxed callback path. The helper calls them synchronously, and pre-cloning a method receiver changes receiver-field initialization semantics.

## Borrow And Evaluation Order

- When assigning into a wrapped target, evaluate the RHS before opening the LHS mutable borrow. Patterns like `x = f(x)` are valid Go and can panic or deadlock if the target is borrowed first.
- Multi-assignment lowering has the same rule: compute moved values into temporaries, then borrow destinations and assign.
- Wrapped map range sources should clone the inner map in a block and drop the borrow guard before the loop body.
- Type-switch lowering should not keep the subject borrow guard alive while executing case bodies. Clone/bind the case value, drop the guard, then emit body statements.
- For pointer-receiver method calls on `self`, stage arguments that read from or call through `self` before starting the mutable receiver borrow.
- For method calls through selector fields, use mutable borrow/lock when go/types says the receiver path calls `.as_mut().unwrap()`.

## Loops, Switches, And Branches

- An unlabeled `break` nested inside `if` / block statements within a switch or type switch breaks that switch, not an outer Rust loop.
- Emit a one-shot labeled loop only for switches that need a synthetic break target. Direct top-level case breaks can still stop case-body emission.
- In `for init; cond; post` loops, unlabeled `continue` runs the loop post before checking the next condition, even when nested inside switch/type-switch blocks.
- Hide outer loop posts inside nested loops that have no post statement.
- Select communication cases should emit synthetic `break` only when the case body can fall through.

## Concurrent Ordering And Helpers

- Struct map-key ordering in concurrent mode must clone field values out before comparison so shared `Arc<Mutex<_>>` keys do not deadlock.
- Package-scoped helper includes share root imports. De-duplicate imports such as `Arc`, `Mutex`, and `Display` instead of moving helpers into local modules.
- Helper types crossing dependency crate boundaries belong in `vendor/go2rust_stdlib_stubs` so generated crates use compatible Rust types.
