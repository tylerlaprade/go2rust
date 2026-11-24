package main

import (
	"path/filepath"
	"strings"
	"testing"
)

// return x in a function with a defer evaluates x, then runs the deferred
// calls, then returns: a deferred x = 2 does not change what is returned.
func TestReturnValueIsStagedBeforeDefersRun(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func f() int {
	x := 1
	defer func() { x = 2 }()
	return x
}

func main() { fmt.Println(f()) }
`)
	staged := strings.Index(rust, "let __return_value =")
	drained := strings.Index(rust, "while let Some(f) = __defer_stack.pop()")
	if staged < 0 || drained < 0 || staged > drained {
		t.Fatalf("the return value should be staged before the defers run:\n%s", rust)
	}
	if !strings.Contains(rust, "return __return_value;") {
		t.Fatalf("the staged value should be returned:\n%s", rust)
	}
}

// recv().add(pair()) calls recv before pair, as Go does: the receiver is
// bound ahead of the multi-value call's results.
func TestMultiValueArgumentBindsCallReceiverFirst(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type acc struct{ n int }

func (a *acc) add(x, y int) { a.n += x + y }

func pair() (int, int) { return 1, 2 }

func recv(a *acc) *acc { return a }

func main() {
	a := &acc{}
	recv(a).add(pair())
}
`)
	receiver := strings.Index(rust, "_r = recv(")
	results := strings.Index(rust, "= pair();")
	if receiver < 0 || results < 0 || receiver > results {
		t.Fatalf("the receiver should be bound before pair() runs:\n%s", rust)
	}
}

// A file of package main other than main.go names main.go's functions and
// types through crate::, since main.go is the crate root.
func TestOtherFilesImportMainFileItems(t *testing.T) {
	tempDir := t.TempDir()
	writeTestFile(t, filepath.Join(tempDir, "go.mod"), "module example.com/rootitems\n\ngo 1.24\n")
	writeTestFile(t, filepath.Join(tempDir, "main.go"), `package main

type point struct{ x, y int }

func divmod(a, b int) (int, int) { return a / b, a % b }

func main() { other() }
`)
	writeTestFile(t, filepath.Join(tempDir, "other.go"), `package main

import "fmt"

func other() {
	q, r := divmod(9, 4)
	fmt.Println(point{q, r})
}
`)
	generator := NewProjectGenerator([]string{filepath.Join(tempDir, "main.go"), filepath.Join(tempDir, "other.go")})
	if err := generator.Generate(); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}
	otherRS := mustReadFile(t, filepath.Join(tempDir, "other.rs"))
	if !strings.Contains(otherRS, "use crate::{divmod, point};") {
		t.Fatalf("other.rs should import main.go's items from the crate root:\n%s", otherRS)
	}
}
