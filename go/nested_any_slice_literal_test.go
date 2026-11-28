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
