package main

import (
	"strings"
	"testing"
)

// The right operand of s[0] + bump(s) runs first, so the read of s[0]
// sees bump's write and does not hold a borrow of s while bump runs.
func TestBinaryOperandReadsAfterTheCallOnItsRight(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func bump(s []int) int { s[0]++; return 1 }

func main() {
	s := []int{1, 2}
	x := s[0] + bump(s)
	_ = x
}
`)
	if !strings.Contains(rust, "let __go_rhs_") {
		t.Fatalf("s[0] + bump(s) should evaluate bump(s) into a temporary first:\n%s", rust)
	}
	call := strings.Index(rust, "bump(s.clone())")
	read := strings.Index(rust, "let __go_binary_value")
	if call < 0 || read < 0 || call > read {
		t.Fatalf("bump(s) should run before s[0] is read:\n%s", rust)
	}
}

// A call cannot change a local variable that no closure captures and whose
// address is never taken, so total + plain(i) keeps its usual form.
func TestBinaryOperandLocalVariableKeepsItsOrder(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func plain(i int) int { return i * 2 }

func main() {
	total := 0
	for i := 0; i < 3; i++ {
		total = total + plain(i)
	}
	_ = total
}
`)
	if strings.Contains(rust, "__go_rhs_") {
		t.Fatalf("total + plain(i) should not use a temporary:\n%s", rust)
	}
}

// A local variable that a closure captures, or whose address is taken,
// can change during a call.
func TestBinaryOperandReachableLocalReadsAfterTheCall(t *testing.T) {
	for name, src := range map[string]string{
		"captured": `package main

func main() {
	c := 0
	next := func() int { c++; return c }
	x := c + next()
	_ = x
}
`,
		"address taken": `package main

func set(p *int) int { *p = 10; return 1 }

func main() {
	n := 3
	x := n + set(&n)
	_ = x
}
`,
	} {
		t.Run(name, func(t *testing.T) {
			rust := transpileTypedRegression(t, src)
			if !strings.Contains(rust, "let __go_rhs_") {
				t.Fatalf("the call should run before the variable is read:\n%s", rust)
			}
		})
	}
}

// && and || run their right operand only when needed, so they keep their
// form.
func TestBinaryOperandShortCircuitKeepsItsOrder(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func bump(s []int) bool { s[0]++; return true }

func main() {
	s := []int{1}
	ok := s[0] > 0 && bump(s)
	_ = ok
}
`)
	if strings.Contains(rust, "__go_rhs_") {
		t.Fatalf("s[0] > 0 && bump(s) should not use a temporary:\n%s", rust)
	}
}
