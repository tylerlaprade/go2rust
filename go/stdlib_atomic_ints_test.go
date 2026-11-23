package main

import (
	"strings"
	"testing"
)

func TestAtomicIntegerOperationsWorkOnThePointedCell(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "sync/atomic"

func bump(p *int32) int32 { return atomic.AddInt32(p, 1) }

func main() {
	var a int32
	var u uint64
	_ = bump(&a)
	_ = atomic.SwapUint64(&u, 3)
	_ = atomic.CompareAndSwapInt32(&a, 1, 2)
	atomic.StoreUint64(&u, 4)
	_ = atomic.LoadUint64(&u)
}
`)
	for _, want := range []string{
		"let __target = p.clone();",
		"*__value = __value.wrapping_add(__delta);",
		"std::mem::replace(__guard.as_mut().unwrap(), __stored)",
		"if *__value == __old { *__value = __new; true } else { false }",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "generic stub function body has no implementation") {
		t.Errorf("an atomic integer operation fell back to the generic stub:\n%s", rust)
	}
}
