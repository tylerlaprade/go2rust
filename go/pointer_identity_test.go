package main

import (
	"strings"
	"testing"
)

func TestStructPointerEqualityComparesTheIdentityFieldCell(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Node struct {
	name   string
	parent *Node
}

func (p *Node) Add(c *Node) { c.parent = p }

func (p *Node) IsParentOf(c *Node) bool { return c.parent == p }

func same(a, b *Node) bool { return a == b }
`)
	// A handle made from the receiver holds a copy that shares the
	// receiver's field cells: pointers compare by the cell of the first
	// field that is always one, name.
	for _, want := range []string{
		"::as_ptr(&__v.name) as *const () as usize",
		"let __self_id = ",
		"__left_id.is_some() && __left_id == __right_id",
		"std::mem::size_of_val(&*__peer)",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("struct pointers should compare by the identity of their first cell field; missing %q:\n%s", want, rust)
		}
	}
}
