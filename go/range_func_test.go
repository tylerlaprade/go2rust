package main

import (
	"strings"
	"testing"
)

func TestRangeOverFuncCallsTheIteratorWithAYieldFunction(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func count(yield func(int) bool) {
	for i := 0; i < 3; i++ {
		if !yield(i) {
			return
		}
	}
}

func first() int {
	for v := range count {
		if v == 1 {
			continue
		}
		if v == 2 {
			return v
		}
	}
	return -1
}

func main() {
	for v := range count {
		if v == 1 {
			break
		}
		fmt.Println(v)
	}
	fmt.Println(first())
}
`)
	if strings.Contains(rust, "type info required for range statement") {
		t.Fatalf("a range over a function should call it:\n%s", rust)
	}
	for _, want := range []string{
		"count(",
		"-> bool",
		"return false;",
		"return true;",
		"__rf_done_",
		"__rf_result_",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("a range over a function should become a call with a yield function; missing %q:\n%s", want, rust)
		}
	}
}

func TestRangeOverFuncTakesABranchToTheLoopAroundIt(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func count(yield func(int) bool) {
	for i := 0; i < 3; i++ {
		if !yield(i) {
			return
		}
	}
}

func main() {
outer:
	for _, row := range []int{1, 2} {
		for v := range count {
			if v == row {
				continue outer
			}
			fmt.Println(row, v)
		}
	}
	var last int
	for last = range count {
	}
	fmt.Println(last)
}
`)
	for _, want := range []string{
		"__rf_next_",
		"continue 'outer",
		"__rf_param_",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("a labeled continue of the loop around a range over a function should stop the iterator and continue that loop; missing %q:\n%s", want, rust)
		}
	}
}

// m = (a[0]) is written as m = a[0] is: the array holds the map bare, and
// the variable is the map's cell.
func TestParenthesizedBareMapElementAssignmentIsWrapped(t *testing.T) {
	src := `package main

import "fmt"

func main() {
	a := [2]map[string]int{{"x": 1}, {"y": 2}}
	m := map[string]int{}
	m = RHS
	fmt.Println(m)
}
`
	plain := transpileTypedRegression(t, strings.Replace(src, "RHS", "a[0]", 1))
	paren := transpileTypedRegression(t, strings.Replace(src, "RHS", "(a[0])", 1))
	if plain != paren {
		t.Fatalf("m = (a[0]) differs from m = a[0]:\n%s\n----\n%s", plain, paren)
	}
}
