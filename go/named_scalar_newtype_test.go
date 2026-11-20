package main

import (
	"strings"
	"testing"
)

func TestNamedFloatArithmeticReadsTheValueItHolds(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Celsius float64
type Fahrenheit float64

func CToF(c Celsius) Fahrenheit { return Fahrenheit(c*9/5 + 32) }

func main() {
	c := Celsius(100)
	fmt.Println(CToF(c), float64(c))
}
`)
	if !strings.Contains(rust, "* 9.0) / 5.0) + 32.0)") {
		t.Fatalf("expected f64 arithmetic on the value inside c:\n%s", rust)
	}
}

func TestNamedStringConstantIsAValueOfItsType(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Color string

const Red Color = "red"

func main() {
	c := Red
	m := map[Color]int{Red: 1}
	fmt.Println(c, m[Red], len(c))
}
`)
	if !strings.Contains(rust, "Color(Rc::new(RefCell::new(Some(RED.to_string()))))") {
		t.Fatalf("expected Red as a Color value:\n%s", rust)
	}
	if !strings.Contains(rust, "impl Ord for Color") {
		t.Fatalf("a named string keys a map, so it needs Ord:\n%s", rust)
	}
}

func TestRangeOverNamedFloatSliceClones(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Celsius float64

func main() {
	var sum Celsius
	for _, t := range []Celsius{1.5, 2.5} {
		sum += t
	}
	fmt.Println(sum)
}
`)
	if strings.Contains(rust, ".iter().copied()") {
		t.Fatalf("Celsius is not Copy:\n%s", rust)
	}
}
