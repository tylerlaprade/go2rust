package main

import (
	"strings"
	"testing"
)

// m.table[k] = func() { ...m... } holds the map handle before the closure
// takes its captured clone of m, so the insert does not read a moved m
// (DEV-189).
func TestMapFieldAssignOfClosureCapturingOwnerHoldsMapFirst(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type M struct {
	table map[int]func()
	n     int
}

func main() {
	m := &M{table: map[int]func(){}}
	m.table[1] = func() { m.n++ }
	m.table[1]()
	println(m.n)
}
`)
	holder := strings.Index(rust, "let __map_holder = ")
	closure := strings.Index(rust, "move ||")
	if holder < 0 || closure < 0 || holder > closure {
		t.Errorf("the map handle should be held before the closure captures m:\n%s", rust)
	}
}

// A defined-type field read through the method receiver is boxed into an
// interface as its value, not as its cell (DEV-189).
func TestReceiverDefinedTypeFieldBoxesValueIntoInterface(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type State int

type M struct{ state State }

func (m *M) get() any { return m.state }

func main() {
	m := &M{state: 1}
	_ = m.get()
}
`)
	if strings.Contains(rust, "Box::new(self.state.clone().clone())") {
		t.Errorf("a receiver field should not be boxed as its cell:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new((*self.state.borrow().as_ref().unwrap()).clone())") {
		t.Errorf("a receiver field should be boxed as its value:\n%s", rust)
	}
}
