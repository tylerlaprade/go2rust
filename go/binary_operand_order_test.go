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

// gc evaluates a builtin call and x && b on the left in order with the
// call on the right, so the left operand goes into a temporary first, which
// also ends its borrows before the call.
func TestBinaryOperandOrderedLeftOperandGoesFirst(t *testing.T) {
	for name, src := range map[string]string{
		"logical": `package main

var flag bool

func raise() bool { flag = true; return true }

func main() {
	yes := true
	ok := (yes && flag) == raise()
	_ = ok
}
`,
		"len": `package main

var text = "a"

func grow() int { text = "abcd"; return 0 }

func main() {
	n := len(text) + grow()
	_ = n
}
`,
	} {
		t.Run(name, func(t *testing.T) {
			rust := transpileTypedRegression(t, src)
			if !strings.Contains(rust, "let __go_lhs_") || strings.Contains(rust, "let __go_rhs_") {
				t.Fatalf("the left operand should go into a temporary before the call:\n%s", rust)
			}
		})
	}
}

// A builtin that writes, such as copy, is ordered like a call.
func TestBinaryOperandCopyOnTheRightCountsAsACall(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func main() {
	dst := []int{1}
	n := dst[0] + copy(dst, []int{5})
	_ = n
}
`)
	if !strings.Contains(rust, "let __go_rhs_") {
		t.Fatalf("copy should run before dst[0] is read:\n%s", rust)
	}
}

// The temporary does not take the name of a variable the other operand
// reads.
func TestBinaryOperandTemporaryAvoidsTheOperandsNames(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func bump(s []int) int { s[0]++; return 1 }

func main() {
	__go_rhs_1 := 10
	s := []int{1}
	p := &__go_rhs_1
	_ = p
	x := __go_rhs_1 + s[0] + bump(s)
	_ = x
}
`)
	if strings.Contains(rust, "let __go_rhs_1 = ") || !strings.Contains(rust, "let __go_rhs_") {
		t.Fatalf("the temporary should not shadow __go_rhs_1:\n%s", rust)
	}
}

// The temporary avoids the names of the operand it stages too, and a name
// that a package variable would make Rust spell differently.
func TestBinaryOperandTemporaryAvoidsStagedAndGlobalNames(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

var limit = 5

var __go_lhs_2 = 0

func one() int { return 1 }

func main() {
	for __go_lhs_1 := range []int{0} {
		println(min(__go_lhs_1, limit) + one())
	}
}
`)
	if strings.Contains(rust, "let __go_lhs_1 = ") || strings.Contains(rust, "let __go_lhs_2 = ") || !strings.Contains(rust, "let __go_lhs_3 = ") {
		t.Fatalf("the temporary should skip __go_lhs_1 and __go_lhs_2:\n%s", rust)
	}
}
