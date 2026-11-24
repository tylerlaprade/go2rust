package main

import (
	"strings"
	"testing"
)

// A nil in a []any element or a ...any argument is a box holding GoNilAny,
// which compares, switches and prints as nil (DEV-151).
func TestNilAnyElementsHoldGoNilAny(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func count(args ...any) int {
	n := 0
	for _, a := range args {
		if a == nil {
			n++
		}
	}
	return n
}

func main() {
	var err error
	var e any
	xs := []any{nil, 1, e}
	xs = append(xs, nil, err)
	fmt.Println(count(xs...), count(nil, 2), xs[0] == nil)
	switch xs[1].(type) {
	case nil:
		fmt.Println("nil")
	}
}
`)
	for _, want := range []string{
		"struct GoNilAny;",
		"vec![Box::new(GoNilAny) as Box<dyn Any>, Box::new(1i64) as Box<dyn Any>, { let __any_holder = e.clone();",
		"None => Box::new(GoNilAny) as Box<dyn Any> } }",
		"(**&a).is::<GoNilAny>()",
		".as_ref().map_or(true, |__any| (**__any).is::<GoNilAny>());",
		"let _ts_is_nil = _ts_guard.as_ref().map_or(true, |__v| (**__v).is::<GoNilAny>());",
		"if let Some(v) = value.downcast_ref::<GoNilAny>() { return Some(Box::new(v.clone())",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "requires nil interface representation") {
		t.Errorf("a nil error should no longer panic on the way into any:\n%s", rust)
	}
}

// append(dst, src...) over []any copies each element with go_any_clone:
// a Box<dyn Any> has no Clone.
func TestAppendSpreadOfAnySliceClonesEachBox(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func main() {
	xs := []any{1, "a"}
	ys := append([]any{"x"}, xs...)
	fmt.Println(len(ys))
}
`)
	if !strings.Contains(rust, ".iter().map(|__go_elem| go_any_clone(__go_elem.as_ref()))") {
		t.Fatalf("each element should be copied with go_any_clone:\n%s", rust)
	}
}

// A type switch over a range variable of a []any tests the borrowed trait
// object for GoNilAny: the variable is an owned clone of the element here,
// so dereferencing it twice does not compile.
func TestTypeSwitchOnAnyRangeVarDetectsNilWithoutDeref(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func classify(args ...any) string {
	out := ""
	for _, arg := range args {
		switch arg.(type) {
		case nil:
			out += "nil "
		case int:
			out += "int "
		}
	}
	return out
}

func main() {
	fmt.Println(classify(nil, 1))
}
`)
	if !strings.Contains(rust, "let _ts_is_nil = _ts_val.map_or(false, |__v| __v.is::<GoNilAny>());") {
		t.Fatalf("the nil case should test the borrowed trait object:\n%s", rust)
	}
	if strings.Contains(rust, "(**_ts_ref)") {
		t.Fatalf("the range variable should not be dereferenced twice:\n%s", rust)
	}
}
