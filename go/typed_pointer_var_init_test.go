package main

import (
	"strings"
	"testing"
)

// var p *T = q copies q's pointer handle instead of wrapping it in another
// cell (DEV-181).
func TestTypedPointerVarInitializerCopiesHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Node struct {
	v    int
	next *Node
}

var head = &Node{v: 1, next: &Node{v: 2}}

func main() {
	n := head
	var p *Node = n.next
	var r *Node = head
	nodes := []*Node{p}
	var first *Node = nodes[0]
	_, _ = r, first
}
`)
	for _, want := range []string{
		"let mut p: Rc<RefCell<Option<Node>>> = (*n.borrow().as_ref().unwrap()).next.clone();",
		"let mut r: Rc<RefCell<Option<Node>>> = (*head.borrow().as_ref().unwrap()).clone();",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q in:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "let mut first: Rc<RefCell<Option<Node>>> = Rc::new(") {
		t.Errorf("a slice element pointer should not be wrapped again:\n%s", rust)
	}
}

// os.Stdout and os.Stderr hand out one shared handle, and comparing them
// compares the handles (DEV-181).
func TestStdStreamSelectorsCompareByPointer(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import (
	"fmt"
	"os"
)

func main() {
	var out *os.File = os.Stdout
	fmt.Println(out == os.Stdout, os.Stdout == os.Stderr)
}
`)
	if !strings.Contains(rust, "let mut out: Rc<RefCell<Option<os_File>>> = os::Stdout().clone();") {
		t.Errorf("var out *os.File = os.Stdout should copy the handle:\n%s", rust)
	}
	if !strings.Contains(rust, "Rc::ptr_eq(&__left, &__right)") {
		t.Errorf("out == os.Stdout should compare the handles:\n%s", rust)
	}
	if !strings.Contains(rust, "thread_local! { static __GO_FILE: Rc<RefCell<Option<os_File>>>") {
		t.Errorf("os.Stdout should be one shared handle:\n%s", rust)
	}
}
