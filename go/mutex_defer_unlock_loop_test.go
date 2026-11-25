package main

import (
	"strings"
	"testing"
)

// A goroutine started in a loop is its own function: its mu.Lock() takes a
// guard, which the dropped deferred Unlock relies on (DEV-178).
func TestGoroutineInLoopLocksWithGuard(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

import "sync"

func main() {
	var mu sync.Mutex
	var wg sync.WaitGroup
	total := 0
	for i := 0; i < 3; i++ {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()
			mu.Lock()
			defer mu.Unlock()
			total += i
		}(i)
	}
	wg.Wait()
}
`)
	if !strings.Contains(rust, "= mu_thread.clone(); let __mutex_guard_") {
		t.Errorf("the goroutine's Lock should take a guard:\n%s", rust)
	}
	if strings.Contains(rust, "mu_thread.lock();") {
		t.Errorf("the goroutine's Lock should not be a plain lock(), whose deferred Unlock is dropped:\n%s", rust)
	}
}

// A function with a defer stack runs a deferred Unlock whose Lock took no
// guard, as one in a loop does.
func TestDeferredUnlockWithoutGuardRuns(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

import (
	"fmt"
	"sync"
)

func main() {
	var mu sync.Mutex
	for i := 0; i < 2; i++ {
		mu.Lock()
		defer mu.Unlock()
		defer fmt.Println(i)
		if i == 0 {
			break
		}
	}
}
`)
	if strings.Contains(rust, "// mu.Unlock() handled by RAII guard") {
		t.Errorf("an Unlock with no guard should be deferred, not dropped:\n%s", rust)
	}
	if !strings.Contains(rust, ".unlock()") {
		t.Errorf("the deferred Unlock should unlock the mutex:\n%s", rust)
	}
}
