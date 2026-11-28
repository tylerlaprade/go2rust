package main

import (
	"strings"
	"testing"
)

// An array of arrays of structs prints each array in brackets of its own
// and each struct through its Display (#345). format_slice needs its
// elements to have a Display, which a Rust array does not.
func TestNestedStructArrayPrints(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type P struct{ X, Y int }

func main() {
	var b [2][2]P
	fmt.Println(b)
	fmt.Printf("%+v\n", b)
}
`)
	if strings.Contains(rust, "format_slice(&b)") {
		t.Fatalf("an array of arrays of structs should not print through format_slice:\n%s", rust)
	}
	for _, want := range []string{
		`__struct_row_1.iter().map(|__struct_row_2| format!("{}", __struct_row_2))`,
		`__struct_row_1.iter().map(|__struct_row_2| format!("{:#}", __struct_row_2))`,
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("expected each struct printed through its Display (%s):\n%s", want, rust)
		}
	}
}
