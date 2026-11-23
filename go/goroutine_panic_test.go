package main

import (
	"strings"
	"testing"
)

// A goroutine runs its deferred calls when it panics, so one of them can
// recover; a panic that nothing recovers ends the program with Go's exit
// status instead of only the goroutine's thread.
func TestGoroutinePanicRunsDefersAndEndsTheProgram(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

import "sync"

func main() {
	var wg sync.WaitGroup
	wg.Add(1)
	go func() {
		defer wg.Done()
		defer func() { _ = recover() }()
		panic("boom")
	}()
	wg.Wait()
}
`)
	for _, want := range []string{
		"if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {",
		"let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {",
		"go_store_panic_payload(__go_panic_payload);",
		`eprintln!("panic: {}", __go_panic_text);`,
		"std::process::exit(2);",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q:\n%s", want, rust)
		}
	}
}
