package main

import (
	"strings"
	"testing"
)

// A []any literal nested in another literal boxes its elements, whether its
// type is elided or written out, as a []any literal of its own does (#343).
func TestNestedAnySliceLiteralBoxesElements(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func main() {
	rows := [][]any{{1, "a"}, {2}}
	typed := [][]any{[]any{1, "a"}}
	grid := [2][]any{{true}, {nil}}
	byKey := map[string][]any{"k": {1}}
	fmt.Println(rows, typed, grid, byKey)
}
`)
	for _, want := range []string{
		`vec![vec![Box::new(1i64) as Box<dyn Any>, Box::new("a".to_string()) as Box<dyn Any>], vec![Box::new(2i64) as Box<dyn Any>]]`,
		`[vec![Box::new(true) as Box<dyn Any>], vec![Box::new(GoNilAny) as Box<dyn Any>]]`,
		`Some(vec![Box::new(1i64) as Box<dyn Any>])`,
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %s:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "format_slice(&rows)") || strings.Contains(rust, `format_map_nil(&byKey`) {
		t.Fatalf("a slice or map of []any should print each []any with format_any_slice:\n%s", rust)
	}
}

// A call in a []any literal boxes the value it returns, not the cell it
// returns it in, and a struct variable goes in as Go's copy (#371).
func TestAnySliceLiteralBoxesCallValuesAndStructCopies(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type pair struct{ N int }

type tally int

func mk() tally { return 4 }

func main() {
	p := pair{1}
	vals := []any{mk(), p}
	fmt.Println(vals)
}
`)
	for _, want := range []string{
		"Box::new((*mk().borrow().as_ref().unwrap()).clone()) as Box<dyn Any>",
		"Box::new((*p.borrow().as_ref().unwrap()).__go_value_clone()) as Box<dyn Any>",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %s:\n%s", want, rust)
		}
	}
}
