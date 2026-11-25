package main

import (
	"strings"
	"testing"
)

// A recovered nil pointer dereference has Go's runtime error text (DEV-192).
func TestRecoveredNilPointerDereferenceHasGoRuntimeErrorText(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Node struct{ val int }

func main() {
	defer func() { fmt.Println(recover()) }()
	var n *Node
	fmt.Println(n.val)
}
`)
	for _, want := range []string{
		"__text == \"called `Option::unwrap()` on a `None` value\"",
		`return "runtime error: invalid memory address or nil pointer dereference".to_string();`,
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q in:\n%s", want, rust)
		}
	}
}
