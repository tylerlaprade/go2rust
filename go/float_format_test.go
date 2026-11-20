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
