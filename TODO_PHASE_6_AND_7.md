# TODO: Phase 6 & 7 Implementation Tasks

## Phase 5 Completion: Panic/Recover Full Support

### Core Panic/Recover Mechanism

- [ ] Implement thread-local panic value storage
  - Store panic values as `Any` type for type flexibility
  - Support both string and non-string panic values
  - Clear panic value after successful recover

- [ ] Wrap function bodies with catch_unwind
  - Detect functions that contain defers with recover
  - Generate catch_unwind wrapper around function body
  - Preserve function return values through unwinding

- [ ] Defer execution during panic
  - Execute defers in LIFO order during unwinding
  - Pass panic value to defers for recover() access
  - Support nested defer execution

### Recover Functionality

- [ ] Implement proper recover() behavior
  - Only works inside deferred functions
  - Returns panic value and stops unwinding
  - Returns nil if no panic is active
  - Clear panic state after successful recover

- [ ] Support re-panicking
  - Allow panic() calls inside recover blocks
  - Propagate new panic value up the stack
  - Maintain defer execution order

### Error Type Handling

- [ ] Fix fmt.Errorf transpilation
  - Generate proper Rust error types
  - Support error wrapping and formatting
  - Handle error interface properly

- [ ] Support panic with different types
  - String literals: `panic("message")`
  - Error values: `panic(err)`
  - Integer values: `panic(42)`
  - Any type: `panic(customStruct)`

### Test Cases to Pass

- [ ] tests/XFAIL/panic_recover - basic panic/recover
- [ ] tests/XFAIL/error_handling - error as panic value
- [ ] tests/XFAIL/errors_custom - custom error types

## Phase 6: Control Flow Extensions

### Select Statements

- [ ] Basic select implementation
  - Parse select statement AST
  - Generate Rust equivalent using channels
  - Support multiple case branches

- [ ] Channel operations in select
  - Send operations: `case ch <- value:`
  - Receive operations: `case v := <-ch:`
  - Receive with ok check: `case v, ok := <-ch:`

- [ ] Default case handling
  - Non-blocking select with default
  - Immediate execution if no channels ready
  - Proper fallthrough to default

- [ ] Select with timeout
  - Support `time.After()` in case
  - Implement timeout channels
  - Clean up timeout resources

- [ ] Random selection
  - When multiple channels ready
  - Fair selection algorithm
  - No starvation of cases

### Goto and Labels

- [ ] Label declaration
  - Parse label statements
  - Track label positions in code
  - Validate label uniqueness

- [ ] Goto statement
  - Parse goto statements
  - Validate goto targets exist
  - Check for invalid jumps (into blocks)

- [ ] Code restructuring for goto
  - Convert goto to Rust loop/break
  - Handle forward jumps
  - Handle backward jumps
  - Preserve execution semantics

- [ ] Labeled break/continue
  - Parse labeled break/continue
  - Map to Rust labeled breaks
  - Validate label targets

### Switch Fallthrough

- [ ] Parse fallthrough statements
  - Detect fallthrough in case blocks
  - Track which cases fall through
  - Validate fallthrough usage

- [ ] Merge case bodies
  - Combine consecutive fallthrough cases
  - Preserve execution order
  - Handle variable scoping

- [ ] Generate correct Rust code
  - Use combined match arms where possible
  - Use explicit control flow where needed
  - Maintain Go semantics

### Test Cases to Pass

- [ ] tests/XFAIL/select_basic - basic select
- [ ] tests/XFAIL/select_statements - complex select
- [ ] tests/XFAIL/fallthrough_switch - switch with fallthrough
- [ ] tests/XFAIL/advanced_control_flow - goto, labels, etc.

## Phase 7: Goroutines and Concurrency

### Basic Goroutines

- [ ] Parse go statements
  - Extract function calls
  - Handle anonymous functions
  - Capture variables properly

- [ ] Generate thread::spawn
  - Move captured variables
  - Handle return values (none for goroutines)
  - Set up thread cleanup

- [ ] Variable capture
  - Clone Arc-wrapped variables
  - Detect concurrent access
  - Switch to Arc<Mutex> for shared data

### Channels

- [ ] Channel creation
  - Parse make(chan T, buffer)
  - Generate mpsc or crossbeam channels
  - Support buffered/unbuffered

- [ ] Channel operations
  - Send: `ch <- value`
  - Receive: `value := <-ch`
  - Close: `close(ch)`
  - Range: `for v := range ch`

- [ ] Channel types
  - Bidirectional: `chan T`
  - Send-only: `chan<- T`
  - Receive-only: `<-chan T`
  - Type conversions

### Synchronization

- [ ] WaitGroup
  - Parse sync.WaitGroup
  - Map to Rust equivalent
  - Handle Add/Done/Wait

- [ ] Mutex
  - Parse sync.Mutex
  - Generate Rust Mutex
  - Handle Lock/Unlock

- [ ] Atomic operations
  - Parse atomic package calls
  - Map to Rust atomics
  - Preserve memory ordering

### Concurrency Detection

- [ ] Improve Arc/Mutex detection
  - Track goroutine usage
  - Track channel usage
  - Track sync package usage
  - Switch wrappers appropriately

### Test Cases to Pass

- [ ] tests/XFAIL/goroutines_basic
- [ ] tests/XFAIL/goroutines_simple
- [ ] tests/XFAIL/channels_basic
- [ ] tests/XFAIL/closing_channels
- [ ] tests/XFAIL/waitgroup_sync
- [ ] tests/XFAIL/atomic_operations

## Priority Order

### Immediate (Unblock more tests)

1. Fix interface range loops (architectural issue)
2. Basic select statements
3. Simple goroutines

### Short-term (Core functionality)

1. Complete panic/recover
2. Channel implementation
3. Switch fallthrough

### Medium-term (Full compatibility)

1. Goto and labels
2. WaitGroup and sync primitives
3. Atomic operations

### Long-term (Optimization)

1. Architectural refactor for selective wrapping
2. Escape analysis
3. Performance optimizations

## Success Metrics

- **Phase 5 Complete**: panic_recover test passes, 55+ tests passing
- **Phase 6 Complete**: All control flow tests pass, 65+ tests passing
- **Phase 7 Complete**: All concurrency tests pass, 75+ tests passing
- **MVP Complete**: 100+ tests passing, can transpile real Go programs
