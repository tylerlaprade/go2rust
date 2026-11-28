package main

import (
	"strings"
	"testing"
)

// A [][]any held in an interface prints each row as a []any prints, so
// format_any has a case for the boxed handle of its rows (#374).
func TestNestedAnySliceInInterfacePrints(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func main() {
	var x any = [][]any{{1, "a"}}
	fmt.Println(x)
}
`)
	if !strings.Contains(rust, "value.downcast_ref::<Rc<RefCell<Option<Vec<Vec<Box<dyn Any>>>>>>>() {\n        { let __any_rows_copy") {
		t.Fatalf("format_any should print a boxed [][]any row by row:\n%s", rust)
	}
}
