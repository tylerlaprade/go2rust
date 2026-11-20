package main

import (
	"strings"
	"testing"
)

func TestMethodThatTestsItsReceiverForNilTakesThePointer(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Node struct {
	Value int
	Next  *Node
}

func (n *Node) Len() int {
	if n == nil {
		return 0
	}
	return 1 + n.Next.Len()
}

func (n *Node) First() int { return n.Value }

func main() {
	var empty *Node
	list := &Node{1, nil}
	fmt.Println(empty.Len(), list.Len(), list.First())
}
`)
	if !strings.Contains(rust, "pub fn __go_nil_recv_node_len(n: Rc<RefCell<Option<Node>>>) -> i64") {
		t.Fatalf("expected a helper that takes the pointer:\n%s", rust)
	}
	if !strings.Contains(rust, "__go_nil_recv_node_len(empty.clone())") {
		t.Fatalf("a call on a pointer should pass the handle to the helper:\n%s", rust)
	}
	if strings.Contains(rust, "__go_nil_recv_node_first") {
		t.Fatalf("a method that does not test its receiver keeps its call:\n%s", rust)
	}
}
