# Phase E: Extend Selective Wrapping (Rearchitecture Stages 2-5)

Extend the VarTable system to unwrap more types. The infrastructure exists (vartable.go with WrapFull/WrapNone/WrapOption), but only interface parameters currently use WrapNone. This phase extends it to primitives, collections, and eventually pointers.

## Current State

- VarTable registers ALL function/method params
- Interface params: WrapNone with `&dyn Trait`
- Everything else: WrapFull (Rc<RefCell<Option<T>>>)
- WrapOption is defined but unused

## Stages (each is a separate session or set of sessions)

### Stage 2: Unwrap Primitive Locals

**Goal**: Local variables of type int, float64, bool, string that never have their address taken use bare types.

1. Build usage analysis: walk function body AST before transpilation
   - Track which variables have `&var` (address taken)
   - Track which variables are assigned `nil`
   - Create `go/analysis.go` for this
2. In `go/stmt.go` at `:=` declarations:
   - If type is primitive AND address not taken AND not nil-assigned → register as WrapNone
3. Update `go/expr.go`: the existing `isVarBare()` checks handle the expression side
4. Update `go/stmt.go`: assignment to bare variables emits `var = value` not `*var.borrow_mut() = Some(value)`

**Test**: This should make existing passing tests generate cleaner code without changing behavior.
Run full `./test.sh` — must stay green. Then check output quality.

### Stage 3: Unwrap Primitive Parameters

**Goal**: Function parameters of primitive type that aren't modified use `&T` or `T`.

This is trickier because it changes function signatures, which affects ALL call sites.

1. Analyze each function: does any param have its address taken or get assigned to?
2. If not: param type becomes `T` (copy types) or `&T` (String, etc.)
3. Call sites must stop wrapping the argument
4. Start with private functions only (public API compatibility)

### Stage 4: Unwrap Collections

**Goal**: `Vec<T>` and `HashMap<K,V>` without Rc<RefCell<Option<>>> when possible.

1. Same analysis: address taken? nil-assigned?
2. Most local slices/maps can be bare `Vec<T>` / `HashMap<K,V>`
3. Access patterns change: `vec[i]` instead of `(*vec.borrow()...)[i]`

### Stage 5: Smart Pointers

**Goal**: `*T` in Go → `Option<Box<T>>` instead of `Rc<RefCell<Option<T>>>` when the pointer doesn't alias.

This is the hardest stage — requires escape analysis across function boundaries.

## Per-Session Workflow

1. Pick the next incomplete stage
2. Read existing vartable.go, analysis.go (if it exists), and relevant expr.go/stmt.go code
3. Implement the analysis + registration changes
4. Run full `./test.sh` after EVERY change — this is refactoring, so regressions are the main risk
5. If a stage is too large for one session, commit partial progress (e.g., just the analysis without the unwrapping)
6. Commit after each stable checkpoint

## Regression Protocol

THIS PHASE HAS THE HIGHEST REGRESSION RISK. Every change affects how variables are accessed across the entire transpiler output.

- Run `./test.sh` after every single edit to go/*.go
- If ANY test fails: `git checkout -- go/` and rethink the approach
- Consider making changes opt-in first (e.g., only for functions with a certain pattern) before making them universal
- Compare generated .rs files before and after to verify changes are correct
