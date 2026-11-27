package main

import (
	"path/filepath"
	"regexp"
	"strings"
	"testing"
)

// These tests cover the transpiler fixes that translating the context
// package from source needed. Each one has a matching end-to-end fixture.

func TestCommaOkTypeAssertionToChannelDowncastsTheHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func check(e any, want chan int) (chan int, bool) {
	f, ok := e.(chan int)
	return f, ok && f == want
}
`)

	if !strings.Contains(rust, "downcast_ref::<GoChannel<i64>>()") {
		t.Fatalf("comma-ok assertion to a channel should downcast to the channel handle:\n%s", rust)
	}
	if !strings.Contains(rust, "(<GoChannel<i64>>::default(), false)") {
		t.Fatalf("failed comma-ok assertion to a channel should yield a nil channel:\n%s", rust)
	}
	if !strings.Contains(rust, "impl<T> PartialEq for GoChannel<T>") || !strings.Contains(rust, "std::sync::Arc::ptr_eq(&self.rx, &other.rx)") {
		t.Fatalf("channel equality should compare the shared channel handle:\n%s", rust)
	}
}

func TestMutexUnlockInTerminatingBranchKeepsTheGuardForTheFallthroughPath(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "sync"

type box struct {
	mu   sync.Mutex
	done bool
}

func (b *box) finish() bool {
	b.mu.Lock()
	if b.done {
		b.mu.Unlock()
		return false
	}
	b.done = true
	b.mu.Unlock()
	return true
}
`)

	drops := regexp.MustCompile(`std::mem::drop\((__mutex_guard_\d+)\);`).FindAllStringSubmatch(rust, -1)
	if len(drops) != 2 || drops[0][1] != drops[1][1] {
		t.Fatalf("both unlocks should drop the one guard from the Lock call (got %v):\n%s", drops, rust)
	}
	if strings.Contains(rust, ".mu.unlock()") {
		t.Fatalf("the fallthrough unlock should drop the guard, not unlock the mutex a second time:\n%s", rust)
	}
}

func TestFuncLitPassedInAssignmentToCapturedPointerClonesInsideTheLiteral(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type timer struct{ fn func() }

type ctx struct {
	t     *timer
	count int
}

func after(f func()) *timer { return &timer{fn: f} }

func run() int {
	c := &ctx{}
	c.t = after(func() { c.count++ })
	c.t.fn()
	return c.count
}
`)

	inner := strings.Index(rust, "let c_closure_clone_closure_clone = c_closure_clone.clone(); Rc::new(move ||")
	if inner == -1 {
		t.Fatalf("closure should move its own clone of the captured pointer:\n%s", rust)
	}
	store := strings.Index(rust, "(*c_closure_clone.borrow_mut().as_mut().unwrap()).t = new_val;")
	if store == -1 || store < inner {
		t.Fatalf("assignment target should still use the outer capture after the closure moves its clone:\n%s", rust)
	}
}

func TestAtomicValueStubChecksStoredTypes(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "sync/atomic"

type c struct{ done atomic.Value }

func (x *c) channel() chan struct{} {
	if d := x.done.Load(); d != nil {
		return d.(chan struct{})
	}
	d := make(chan struct{})
	x.done.Store(d)
	return d
}
`)

	for _, want := range []string{
		"pub fn compare_and_swap",
		"pub fn swap",
		`panic!("sync/atomic: store of nil value into Value")`,
		`panic!("sync/atomic: store of inconsistently typed value into Value")`,
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("atomic.Value stub should contain %q:\n%s", want, rust)
		}
	}
}

func TestTypeSwitchLocalInterfaceCaseIsImplementedForImportedStdlibTypes(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), "module example.com/stringify\n\ngo 1.24\n")
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

import (
	"fmt"
	"time"
)

type stringer interface{ String() string }

func stringify(v any) string {
	if s, ok := v.(stringer); ok {
		return s.String()
	}
	switch s := v.(type) {
	case stringer:
		return s.String()
	}
	return "?"
}

func main() {
	fmt.Println(stringify(time.Second))
}
`)

	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go")})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	mainRS := mustReadFile(t, filepath.Join(tempDir, "main.rs"))
	if !strings.Contains(mainRS, "impl stringer for time::r#mod::Duration") {
		t.Fatalf("local interface used in a type switch should be implemented for imported stdlib types:\n%s", mainRS)
	}
	if !strings.Contains(mainRS, "time::r#mod::Duration::string(self)") && !strings.Contains(mainRS, "<time::r#mod::Duration>::string(self") {
		t.Fatalf("external local interface methods should call the inherent method explicitly, not recurse:\n%s", mainRS)
	}
}
