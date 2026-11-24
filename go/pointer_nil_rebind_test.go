package main

import (
	"strings"
	"testing"
)

// Assigning nil to a pointer replaces its handle with an empty one instead
// of emptying the pointee's cell, which other pointers share (DEV-142).
func TestPointerNilAssignmentRebindsHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type node struct{ next *node }

func main() {
	value := 1
	items := []*int{&value}
	for _, item := range items {
		item = nil
		fmt.Println(item == nil)
	}
	p := &value
	q := p
	q = nil
	n := &node{}
	n.next = nil
	fmt.Println(*items[0], p == nil, q == nil, n.next == nil)
}
`)
	for _, want := range []string{
		"{ let new_val = Rc::new(RefCell::new(None)); item = new_val; }",
		"{ let new_val = Rc::new(RefCell::new(None)); q = new_val; }",
		".next = new_val;",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q:\n%s", want, rust)
		}
	}
	for _, cleared := range []string{"*item.borrow_mut() = None", "*q.borrow_mut() = None"} {
		if strings.Contains(rust, cleared) {
			t.Errorf("nil should not empty the shared pointee, found %q:\n%s", cleared, rust)
		}
	}
}
