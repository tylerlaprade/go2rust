package main

import (
	"strings"
	"testing"
)

// atomic.Int64, atomic.Uint32 and atomic.Bool get working stubs on
// std::sync::atomic instead of the generic stub, whose methods panic
// (DEV-193).
func TestTypedAtomicStubsHaveWorkingMethods(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import (
	"fmt"
	"sync/atomic"
)

func main() {
	var n atomic.Int64
	var u atomic.Uint32
	var b atomic.Bool
	n.Add(1)
	u.Store(2)
	b.Store(true)
	fmt.Println(n.Load(), u.Load(), b.Load())
}
`)
	for _, want := range []string{
		"__go_value: std::sync::Arc<std::sync::atomic::AtomicI64>,",
		"__go_value: std::sync::Arc<std::sync::atomic::AtomicU32>,",
		"__go_value: std::sync::Arc<std::sync::atomic::AtomicBool>,",
		"let previous = self.__go_value.fetch_add(delta, std::sync::atomic::Ordering::SeqCst);",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q in:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "generic stub method body has no implementation") {
		t.Errorf("a typed atomic should not use the generic stub:\n%s", rust)
	}
}
