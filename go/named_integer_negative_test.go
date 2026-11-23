package main

import (
	"strings"
	"testing"
)

func TestNamedIntegerFromNegativeConstantKeepsItsOperand(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Level int
type I8 int8

const step = 5

func main() {
	a := I8(-2)
	b := Level(-step)
	var c any = Level(-4)
	fmt.Println(a, b, c, Level(-3+1))
}
`)
	if strings.Contains(rust, "- as") {
		t.Fatalf("negative constant lost its operand:\n%s", rust)
	}
	for _, want := range []string{"Some(-2 as i8)", "Some(-4 as i64)"} {
		if !strings.Contains(rust, want) {
			t.Fatalf("expected %q:\n%s", want, rust)
		}
	}
}
