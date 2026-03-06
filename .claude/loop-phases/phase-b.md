# Phase B: Concurrency Subsystem

Build channels, goroutines, and select from scratch. This is NOT incremental test-fixing — it requires designing a subsystem first, then implementing it, then running tests.

## Target Tests (~25-30)

goroutines_basic, goroutines_simple, channels_basic, channels_simple,
closing_channels, channel_buffering, channel_directions, channel_sync,
non_blocking_channels, range_over_channels, select_basic, select_statements,
waitgroup_sync, mutex_counter, atomic_operations, shared_mutation,
concurrency_patterns, stateful_goroutines, worker_pools,
tickers_basic, timers_basic, timeouts_basic, rate_limiting

## Session 1: Design

Do NOT write code yet. Instead:

1. Read 5-6 of the target test `main.go` files to understand the Go patterns used
2. Read `go/stmt.go` and `go/expr.go` to understand how new statement/expression types are added
3. Read `go/concurrency.go` if it exists — the transpiler already detects concurrency for Arc/Mutex switching
4. Design the mapping:
   - `go func() { ... }()` → `thread::spawn(move || { ... })`
   - `make(chan T)` / `make(chan T, n)` → `std::sync::mpsc::channel()` or `sync_channel(n)`
   - `ch <- val` → `tx.send(val)`
   - `val := <-ch` → `rx.recv()`
   - `close(ch)` → `drop(tx)`
   - `for val := range ch` → `for val in rx.iter()`
   - `select { case ... }` → needs a macro or manual poll
   - `sync.WaitGroup` → `std::sync::Barrier` or manual `Arc<AtomicUsize>` + `Condvar`
   - `sync.Mutex` → `std::sync::Mutex`
5. Write the design to a file: `go/CONCURRENCY_DESIGN.md`
6. Commit the design file

## Sessions 2-3: Core Implementation

Implement in this order (each step should leave tests passing):

1. **Channel creation**: `make(chan T)` → generate channel type + creation code
2. **Send/receive**: `ch <- v` and `v := <-ch` → `tx.send()` / `rx.recv()`
3. **Goroutines**: `go func(){}()` → `thread::spawn` with proper variable capture (clone Arc-wrapped vars)
4. **Close**: `close(ch)` → `drop(tx)`
5. **Range over channel**: `for v := range ch` → `for v in rx.iter()`
6. Run `./test.sh goroutines_basic` and `./test.sh channels_basic` — iterate until they pass

## Sessions 4+: Extended Features

7. **Buffered channels**: `make(chan T, n)` → `sync_channel(n)`
8. **Channel directions**: `chan<- T`, `<-chan T` type constraints
9. **Select**: Start with simple 2-case select, then default case
10. **WaitGroup**: `sync.WaitGroup` → custom or `Arc<(Mutex<i32>, Condvar)>`
11. **Mutex**: `sync.Mutex` → `std::sync::Mutex`
12. Run remaining concurrency tests

## Regression Protocol

- Build and run full `./test.sh` after EVERY change to `go/*.go`
- If any previously-passing test fails: `git checkout -- go/` and rethink
- Concurrency changes should NOT affect single-threaded code paths
- The `NeedsConcurrentWrapper()` detection already exists — use it

## Key Files to Modify

- `go/stmt.go` — go statements, channel send, select
- `go/expr.go` — channel receive (`<-ch`), make(chan)
- `go/types.go` — channel type mapping
- `go/stdlib.go` — sync package functions (WaitGroup, Mutex)
- `go/concurrency.go` — existing concurrency detection (extend it)
