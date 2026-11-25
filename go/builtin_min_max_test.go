package main

import (
	"strings"
	"testing"
)

func TestBuiltinMinMaxOfNamedScalarsWrapsTheHeldValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Celsius float64

type Level int

func hottest(a, b Celsius) Celsius {
	return max(a, b, 0)
}

func lowest(l Level) Level {
	return min(l, 7)
}
`)
	if strings.Contains(rust, "as f64)).max(") {
		t.Fatalf("min and max of named scalars should not cast the newtype:\n%s", rust)
	}
	if !strings.Contains(rust, "Celsius(Rc::new(RefCell::new(Some({ let __l = ") {
		t.Fatalf("max of Celsius values should be a Celsius:\n%s", rust)
	}
	if !strings.Contains(rust, "Level(Rc::new(RefCell::new(Some(std::cmp::min(") || !strings.Contains(rust, "(7 as i64)") {
		t.Fatalf("min of a Level and a constant should be a Level over the held values:\n%s", rust)
	}
}

func TestBuiltinFloatMinMaxFollowsGoForNaNAndZeros(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func pick(a, b float64) (float64, float64) {
	return max(a, b), min(a, b)
}
`)
	for _, want := range []string{
		"if __l.is_nan() || __r.is_nan() { __l + __r }",
		"__r > __l || (__r == __l && __r.is_sign_positive())",
		"__r < __l || (__r == __l && __r.is_sign_negative())",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("float min and max should contain %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, ").max(") || strings.Contains(rust, ").min(") {
		t.Fatalf("Rust's f64::max drops a NaN; Go's max returns it:\n%s", rust)
	}
}

func TestClearOfSliceExpressionReachesCapacity(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func shrink(s []int, n int) []int {
	old := len(s)
	s = s[:n]
	clear(s[len(s):old])
	return s
}
`)
	if !strings.Contains(rust, "__clear_seq.reslice3_zeroed(__clear_start, __clear_end, __clear_cap)") {
		t.Fatalf("clear(s[lo:hi]) should clear a window of s's backing up to its capacity:\n%s", rust)
	}
	if strings.Contains(rust, "__clear_end <= __clear_seq.len()") {
		t.Fatalf("clear(s[lo:hi]) should not require hi <= len(s):\n%s", rust)
	}
}

func TestAppendToNamedSliceLiteralUsesTheNewtypeValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type IDs []int

func clone(s IDs) IDs {
	return append(IDs{}, s...)
}
`)
	if strings.Contains(rust, "(*IDs(") {
		t.Fatalf("IDs{} has no cell to read through:\n%s", rust)
	}
	if !strings.Contains(rust, "{ let __named_slice = IDs(") {
		t.Fatalf("the base of the append should be the IDs value:\n%s", rust)
	}
}

func TestTypeVerbOfConcreteCallResultIsNamedStatically(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type IDs []int

func make3() IDs { return IDs{1, 2, 3} }

func main() {
	fmt.Printf("%T\n", make3())
}
`)
	if !strings.Contains(rust, `{ let _ = make3(); "main.IDs" }`) {
		t.Fatalf("%%T of a call with a concrete result type should evaluate the call and name its type:\n%s", rust)
	}
}
