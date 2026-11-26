package main

import (
	"strings"
	"testing"
)

// A plain operand that a later call can change is read after the call.
func TestFmtOperandsWithCallsAreBoundFirst(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type box struct{ n int }

func (b *box) inc() int { b.n++; return b.n }

func main() {
	b := &box{}
	fmt.Println(b.n, b.inc())
}
`)
	if !strings.Contains(rust, "let __go_print_arg_") {
		t.Fatalf("b.inc() should be bound before b.n is printed:\n%s", rust)
	}
}

// A plain local that no call can reach keeps the usual print.
func TestFmtOperandsUnreachableLocalKeepsItsPlace(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import (
	"fmt"
	"strconv"
)

func main() {
	n := 2
	path := []byte{'a'}
	fmt.Println(n+1, strconv.Itoa(n*2), path[0], classify(path[0]))
}

func classify(b byte) string { return string(b) }
`)
	if strings.Contains(rust, "__go_print_arg_") {
		t.Fatalf("no operand needs binding first:\n%s", rust)
	}
}
