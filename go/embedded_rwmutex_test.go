package main

import (
	"strings"
	"testing"
)

// A struct embedding sync.RWMutex gets forwarders of its methods that call
// the bare GoRWMutex, and a struct embedding that struct forwards them
// through a clone of the mutex taken without holding the embedded struct's
// guard, so a forwarder that waits for the lock holds no other lock.
func TestEmbeddedRWMutexMethodsAreForwarded(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "sync"

type Table struct {
	sync.RWMutex
	m map[string]int
}

func (t *Table) Read(k string) int {
	t.RLock()
	defer t.RUnlock()
	return t.m[k]
}

type Registry struct {
	*Table
}

func main() {
	r := Registry{Table: &Table{m: map[string]int{}}}
	r.Lock()
	r.Unlock()
	_ = r.Read("a")
}
`)
	for _, want := range []string{
		"pub fn r_lock(&self) {\n        let embedded_ref = &self.r_w_mutex;\n        embedded_ref.r_lock()",
		"pub fn __go_rw_mutex(&self) -> GoRWMutex {\n        let embedded = self.table.clone();",
		"pub fn lock(&self) {\n        self.__go_rw_mutex().lock()",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "self.r_w_mutex.clone(); let mut __promoted_guard") {
		t.Fatalf("the embedded RWMutex is held bare and is not locked as a cell:\n%s", rust)
	}
}

// A directly embedded sync.RWMutex is at depth zero and wins over the one a
// struct embedded beside it has, whatever the field order.
func TestDirectlyEmbeddedRWMutexWinsOverPromotedOne(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "sync"

type Table struct {
	sync.RWMutex
}

type Both struct {
	*Table
	sync.RWMutex
}

func (b *Both) Hold() {
	b.Lock()
	b.Unlock()
}
`)
	start := strings.Index(rust, "impl Both {")
	if start < 0 {
		t.Fatalf("missing impl Both:\n%s", rust)
	}
	both := rust[start:]
	if !strings.Contains(both, "pub fn lock(&self) {\n        let embedded_ref = &self.r_w_mutex;\n        embedded_ref.lock()") {
		t.Fatalf("Both's lock should be its own mutex's:\n%s", both)
	}
	if strings.Contains(both, "fn lock(&self) {\n        self.__go_rw_mutex().lock()") {
		t.Fatalf("Both's lock should not go through Table:\n%s", both)
	}
}

// Go promotes a sync.RWMutex method from the shallowest embedded field, so
// with Outer{ *Middle; *Shallow } and Middle's mutex one level deeper than
// Shallow's, Outer's forwarders go through Shallow even though Middle comes
// first.
func TestRWMutexMethodsComeFromTheShallowestEmbeddedField(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "sync"

type Deep struct {
	sync.RWMutex
}

type Middle struct {
	*Deep
}

type Shallow struct {
	sync.RWMutex
}

type Outer struct {
	*Middle
	*Shallow
}

func main() {
	o := Outer{Middle: &Middle{Deep: &Deep{}}, Shallow: &Shallow{}}
	o.Lock()
	o.Shallow.Unlock()
}
`)
	start := strings.Index(rust, "impl Outer {\n    pub fn __go_rw_mutex")
	if start < 0 {
		t.Fatalf("missing Outer's RWMutex forwarders:\n%s", rust)
	}
	outer := rust[start:]
	if end := strings.Index(outer[1:], "\nimpl "); end >= 0 {
		outer = outer[:end+1]
	}
	if !strings.Contains(outer, "let embedded = self.shallow.clone();") {
		t.Fatalf("Outer's mutex should be Shallow's:\n%s", outer)
	}
	if strings.Contains(outer, "self.middle.clone()") {
		t.Fatalf("Outer's mutex should not come through Middle:\n%s", outer)
	}
}

// r.Lock() on a struct variable whose type embeds sync.RWMutex takes the
// mutex out of r's cell in a block of its own and then waits on it, so a
// goroutine waiting for the lock holds nothing the owner needs to reach
// r.Unlock() through the same variable.
func TestForwardedRWMutexCallOnAVariableReleasesItsCell(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "sync"

type Table struct {
	sync.RWMutex
	m map[string]int
}

type Registry struct {
	*Table
}

func main() {
	r := Registry{Table: &Table{m: map[string]int{}}}
	var wg sync.WaitGroup
	wg.Add(1)
	go func() {
		defer wg.Done()
		r.Lock()
		r.m["a"] = 1
		r.Unlock()
	}()
	wg.Wait()
}
`)
	for _, want := range []string{
		".__go_rw_mutex(); __sync_value }.lock()",
		".__go_rw_mutex(); __sync_value }.unlock()",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, ".as_mut().unwrap()).lock()") || strings.Contains(rust, ".as_ref().unwrap()).lock()") {
		t.Fatalf("Lock is called holding the variable's cell:\n%s", rust)
	}
}

// When go/types cannot say which embedded field a sync.RWMutex method is
// promoted through, its forwarder fails loudly instead of locking whichever
// mutex comes first.
func TestRWMutexForwarderWithoutSelectionIsUnimplemented(t *testing.T) {
	methods := rwMutexForwardersForSelection([]externalPromotedMethod{
		{RustMethodName: "lock", ThroughRWMutexAccessor: true},
		{RustMethodName: rwMutexAccessorName, RWMutexAccessor: true},
	}, false)
	var out strings.Builder
	for _, method := range methods {
		generateExternalPromotedMethod(&out, method)
	}
	rust := out.String()
	for _, want := range []string{
		"pub fn lock(&self) {\n        unimplemented!(\"type info required for promoting sync.RWMutex methods\")",
		"pub fn __go_rw_mutex(&self) -> GoRWMutex {\n        unimplemented!(",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q:\n%s", want, rust)
		}
	}
	if kept := rwMutexForwardersForSelection([]externalPromotedMethod{{RustMethodName: "lock"}}, true); kept[0].Unimplemented != "" {
		t.Fatalf("a known selection should keep its forwarder: %+v", kept[0])
	}
}
