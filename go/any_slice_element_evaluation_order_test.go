package main

import (
	"strings"
	"testing"
)

// Reading an element of a []any element evaluates the row before the
// column, left to right as in Go (#373): the row's holder is bound before
// the index, and the row is borrowed after both.
func TestAnySliceElementEvaluatesRowBeforeColumn(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func row() int { return 0 }

func col() int { return 1 }

func main() {
	rows := [][]any{{1, "a"}}
	s := rows[row()][col()].(string)
	t, ok := rows[row()][col()].(string)
	var v any = rows[row()][col()]
	fmt.Println(s, t, ok, v)
}
`)
	body := rust[strings.Index(rust, "fn main()"):]
	holders := strings.Count(body, "let __seq_holder = ")
	if holders < 3 {
		t.Fatalf("expected a row holder per element read, found %d:\n%s", holders, body)
	}
	for rest := body; strings.Contains(rest, "let __seq_holder = "); {
		start := strings.Index(rest, "let __seq_holder = ")
		rest = rest[start:]
		holder := strings.Index(rest, "row()")
		index := strings.Index(rest, "col()")
		guard := strings.Index(rest, "let __seq_guard = ")
		if holder < 0 || index < 0 || guard < 0 || !(holder < index && index < guard) {
			t.Fatalf("the row must be evaluated before the column, and borrowed after both:\n%s", rest)
		}
		rest = rest[guard:]
	}
}
