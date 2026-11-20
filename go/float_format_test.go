package main

import (
	"strings"
	"testing"
)

func TestDefaultFloatPrintingUsesGoMagnitudeRules(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func main() {
	large := 1000000.0
	small := 0.00001
	var single float32 = 0.000001
	var boxed any = large
	values := []float64{large}
	lookup := map[string]float64{"large": large}
	fmt.Println(large, small, single, boxed)
	fmt.Println(values, lookup)
	fmt.Printf("%v %g %.1f\n", large, large, large)
}
`)

	for _, want := range []string{
		"go_format_float64(",
		"go_format_float32(",
		"go_format_float64(*v)",
		"format_float64_slice",
		"format_float64_map_values",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q in generated Rust:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, `format!("{:.1}", go_format_float64(`) {
		t.Fatalf("explicit %%f formatting must keep the numeric value:\n%s", rust)
	}
}

func TestDefaultFloatFormattingDoesNotChangeBuiltinsOrFlaggedVerbs(t *testing.T) {
	builtinRust := transpileTypedRegression(t, `package main
func main() {
	value := 1000000.0
	println(value)
}
`)
	if strings.Contains(builtinRust, "go_format_float64(") {
		t.Fatalf("builtin println must keep its existing numeric formatting path:\n%s", builtinRust)
	}

	flaggedRust := transpileTypedRegression(t, `package main

import "fmt"

func main() {
	value := 1000000.0
	fmt.Printf("%#v\n", value)
}
`)
	if strings.Contains(flaggedRust, `format!("{:?}", go_format_float64(`) {
		t.Fatalf("%%#v must not debug-format the helper's String and add quotes:\n%s", flaggedRust)
	}
}

func TestFloat32CollectionLiteralKeepsElementTypeWithoutValueUse(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func main() {
	values := []float32{0.1, 2.5}
	fmt.Printf("%T\n", values[0])
}
`)

	if !strings.Contains(rust, "vec![0.1f32, 2.5f32]") {
		t.Fatalf("float32 collection constants must constrain Rust's element type:\n%s", rust)
	}
}

func TestFloatFormattingPreservesErrorMethodAndAssertedSliceHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type floatError float64

func (floatError) Error() string { return "failed" }

func main() {
	fmt.Println(floatError(1000000))
	var value any = []float64{1000000}
	fmt.Println(value.([]float64))
}
`)

	mainBody := rust[strings.LastIndex(rust, "fn main()"):]
	if strings.Contains(mainBody, "go_format_float64(1e+06)") {
		t.Fatalf("named float Error method must take precedence over numeric formatting:\n%s", rust)
	}
	if !strings.Contains(mainBody, "format_float64_slice(&({ let val = value.clone()") {
		t.Fatalf("asserted float slice must pass its wrapped handle to the formatter:\n%s", rust)
	}
}

func TestFloatFormattingPreservesAnySliceElementCollectionHandles(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func main() {
	values := []any{
		[]float64{1000000},
		map[string]float64{"large": 1000000},
	}
	fmt.Println(values[0].([]float64))
	fmt.Println(values[1].(map[string]float64))
}
`)

	mainBody := rust[strings.LastIndex(rust, "fn main()"):]
	if !strings.Contains(mainBody, "format_float64_slice(&({") ||
		!strings.Contains(mainBody, "format_float64_map_values(&({") {
		t.Fatalf("[]any element assertions must pass their collection handles to float formatters:\n%s", rust)
	}
}
