package main

import (
	"strings"
	"testing"
)

// Reading an element of a []any element makes the row's call before the
// column's, left to right as in Go, and reads the row after both, as gc
// does (#373): pickrow() is bound first, then pickcol(), then the row's holder.
func TestAnySliceElementEvaluatesRowBeforeColumn(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func pickrow() int { return 0 }

func pickcol() int { return 1 }

func main() {
	rows := [][]any{{1, "a"}}
	s := rows[pickrow()][pickcol()].(string)
	t, ok := rows[pickrow()][pickcol()].(string)
	var v any = rows[pickrow()][pickcol()]
	fmt.Println(s, t, ok, v)
}
`)
	body := rust[strings.Index(rust, "fn main()"):]
	if rows, cols := strings.Count(body, "pickrow()"), strings.Count(body, "pickcol()"); rows != 3 || cols != 3 {
		t.Fatalf("each read should call pickrow and pickcol once, found %d and %d:\n%s", rows, cols, body)
	}
	rest := body
	for i := 0; i < 3; i++ {
		row := strings.Index(rest, "pickrow()")
		col := strings.Index(rest, "pickcol()")
		holder := strings.Index(rest, "let __seq_holder = ")
		guard := strings.Index(rest, "let __seq_guard = ")
		if !(row < col && col < holder && holder < guard) {
			t.Fatalf("read %d should call pickrow, then pickcol, then read the row and borrow it:\n%s", i+1, rest)
		}
		rest = rest[guard+1:]
	}
}

// A column call that replaces the row is read after it, as gc does: the
// row's holder is bound from the row's index after both calls.
func TestAnySliceElementReadsTheRowAfterTheColumnCall(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

var rows = [][]any{{1}}

func pickrow() int { return 0 }

func pickcol() int {
	rows[0] = []any{2}
	return 0
}

func main() {
	fmt.Println(rows[pickrow()][pickcol()].(int))
	fmt.Println(rows[0][pickcol()].(int))
}
`)
	body := rust[strings.Index(rust, "fn main()"):]
	staged := strings.Index(body, "let __seq_operand_0 = pickrow();")
	if staged < 0 {
		t.Fatalf("pickrow() should be bound before the column call:\n%s", body)
	}
	if !strings.Contains(body[staged:], "[(__seq_operand_0) as usize]") {
		t.Fatalf("the row should be read with pickrow()'s bound result:\n%s", body)
	}
	constant := strings.Index(body[staged+1:], "let __idx = ")
	if constant < 0 || strings.Index(body[staged+1+constant:], "let __seq_holder = ") < 0 {
		t.Fatalf("rows[0][pickcol()] should bind the column before the row:\n%s", body)
	}
}

// A conversion in the row's index is not a call: int(i) reads i with the
// other variables, after the column's call, so the row index is not bound
// before it.
func TestAnySliceElementDoesNotStageAConversion(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

var i = 0

var rows = [][]any{{1}, {2}}

func pickcol() int {
	i = 1
	return 0
}

func main() {
	fmt.Println(rows[int(i)][pickcol()].(int))
}
`)
	body := rust[strings.Index(rust, "fn main()"):]
	if strings.Contains(body, "__seq_operand_") {
		t.Fatalf("a conversion should not be bound before the column:\n%s", body)
	}
	col := strings.Index(body, "pickcol()")
	holder := strings.Index(body, "let __seq_holder = ")
	if col < 0 || holder < 0 || col > holder {
		t.Fatalf("the column should be bound before the row is read:\n%s", body)
	}
}
