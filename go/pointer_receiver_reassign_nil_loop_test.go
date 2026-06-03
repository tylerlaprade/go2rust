package main

import (
	"strings"
	"testing"
)

func TestPointerReceiverReassignmentPreservesNilLoop(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type node struct {
	next *node
	v int
}

func (x *node) walk() int {
	sum := 0
	for x != nil {
		sum += x.v
		x = x.next
	}
	return sum
}
`)

	if strings.Contains(rust, "while true") {
		t.Fatalf("pointer receiver nil loop should preserve the nil condition:\n%s", rust)
	}
	if strings.Contains(rust, "take().unwrap()") {
		t.Fatalf("pointer receiver reassignment should preserve nil instead of unwrapping it:\n%s", rust)
	}
}

func TestPointerReceiverReassignmentPreservesMethodCallAndReturn(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type node struct {
	next *node
	v int
}

func (x *node) value() int {
	return x.v
}

func (x *node) find(want int) *node {
	for x != nil {
		if x.value() == want {
			return x
		}
		x = x.next
	}
	return nil
}
`)

	if strings.Contains(rust, "__self.value(") {
		t.Fatalf("pointer receiver method call should borrow through the nil-preserving handle:\n%s", rust)
	}
	if strings.Contains(rust, "Some(__self.clone())") {
		t.Fatalf("pointer receiver return should clone the nil-preserving handle directly:\n%s", rust)
	}
}
