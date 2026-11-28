package main

import (
	"strings"
	"testing"
)

// A slice of a defined basic type held in an interface prints through
// format_slice, as each element's Display prints it (#358).
func TestDefinedBasicSliceInInterfacePrints(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Level int

func main() {
	var x any = []Level{1, 2}
	fmt.Println(x)
}
`)
	if !strings.Contains(rust, "value.downcast_ref::<Rc<RefCell<Option<Vec<Level>>>>>() {\n        format_slice(v)") {
		t.Fatalf("format_any should print a []Level with format_slice:\n%s", rust)
	}
}
