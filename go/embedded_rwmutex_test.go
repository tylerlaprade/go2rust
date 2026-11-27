package main

import (
	"strings"
	"testing"
)

// A struct embedding sync.RWMutex gets forwarders of its methods that call
// the bare GoRWMutex, and a struct embedding that struct forwards them
// through it.
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
		"pub fn lock(&self) {\n        let embedded = self.table.clone();",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "self.r_w_mutex.clone(); let mut __promoted_guard") {
		t.Fatalf("the embedded RWMutex is held bare and is not locked as a cell:\n%s", rust)
	}
}
