package main

import (
	"strings"
	"testing"
)

// s[i].Next = &s[j] writes the element's GoPtr field in place. The GoPtr
// field is a plain Rust field, so a write to a clone of the element was lost
// (DEV-156).
func TestSliceElemGoPtrFieldAssignmentWritesElementInPlace(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Node struct {
	Val  int
	Next *Node
}

func main() {
	nodes := []Node{{}, {}, {}}
	for i := 1; i < len(nodes); i++ {
		nodes[i-1].Next = &nodes[i]
	}
	for n := &nodes[0]; n != nil; n = n.Next {
		fmt.Print(n.Val)
	}
}
`)
	want := "{ let __idx = ((*i.borrow().as_ref().unwrap()) - 1) as usize; let new_val = GoPtr::slice_elem("
	if !strings.Contains(rust, want) {
		t.Errorf("the index should be evaluated before the value (%q):\n%s", want, rust)
	}
	want = "let __seq_handle = nodes.clone(); let mut __seq_guard = __seq_handle.borrow_mut(); let __len = __seq_guard.as_ref().map(|__seq| __seq.len()).unwrap_or(0); if __idx >= __len { std::mem::drop(__seq_guard); panic!("
	if !strings.Contains(rust, want) {
		t.Errorf("an index out of range should unlock the slice before it panics (%q):\n%s", want, rust)
	}
	if !strings.Contains(rust, "(*__seq_guard.as_mut().unwrap())[__idx].next = new_val; }") {
		t.Errorf("missing the in-place field write:\n%s", rust)
	}
	if strings.Contains(rust, ".clone().next = new_val") {
		t.Errorf("the field write should not go to a clone of the element:\n%s", rust)
	}
	if strings.Contains(rust, "let mut n: Option<GoSliceElemPtr<") {
		t.Errorf("a loop pointer that follows a GoPtr field should be a GoPtr local:\n%s", rust)
	}
}

// arr[0].Next.Next opens the pointer after the array's borrow is released:
// the pointer points into the same array, and opening it for writing under
// that borrow panicked (DEV-156).
func TestArrayElemGoPtrFieldPointeeReadReleasesArrayBorrow(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Node struct {
	Val  int
	Next *Node
}

func main() {
	var arr [3]Node
	for i := 0; i < 2; i++ {
		arr[i].Next = &arr[i+1]
	}
	fmt.Println(arr[0].Next.Next.Val)
}
`)
	want := "{ let __ptr_base = (*arr.borrow().as_ref().unwrap())[(0) as usize].clone().next.clone(); __ptr_base }.with_mut("
	if !strings.Contains(rust, want) {
		t.Errorf("missing %q:\n%s", want, rust)
	}
}
