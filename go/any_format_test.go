package main

import (
	"strings"
	"testing"
)

// A pointer to a struct prints expanded as an argument of a print call and as
// its address inside another value. Each place that prints says which it is.
// Nothing is remembered between print calls, so a String method that prints
// starts at the top level again.
func TestFormatAnyTellsNestedFromTopLevelWithoutKeepingState(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Point struct{ X int }

type Row struct{ Value interface{} }

func show(v interface{}) { fmt.Println(v) }

func main() {
	show(&Point{1})
	fmt.Println(Row{Value: 2})
}
`)

	for _, want := range []string{
		"fn format_any(value: &dyn Any) -> String {\n    format_any_at(value, false)\n}",
		"fn format_any_nested(value: &dyn Any) -> String {\n    format_any_at(value, true)\n}",
		"fn format_any_at(value: &dyn Any, nested: bool) -> String {",
		"go_format_any_pointer(nested, Rc::as_ptr(v) as *const (), ",
		"Some(__v) => format_any_nested(__v.as_ref()), None => \"<nil>\".to_string()",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q in:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "thread_local!") {
		t.Fatalf("the printer must not keep its position between calls:\n%s", rust)
	}
}

// Without a pointer case nothing prints differently inside another value, so
// format_any keeps its plain form and format_any_nested only forwards to it.
func TestFormatAnyNestedForwardsWhenNoPointerIsBoxed(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Row struct{ Value interface{} }

func main() {
	fmt.Println(Row{Value: 2})
}
`)

	for _, want := range []string{
		"\nfn format_any(value: &dyn Any) -> String {\n    if let Some(v) = value.downcast_ref::<i32>() {",
		"fn format_any_nested(value: &dyn Any) -> String {\n    format_any(value)\n}",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q in:\n%s", want, rust)
		}
	}
	for _, unwanted := range []string{"format_any_at", "go_format_any_pointer", "nested: bool"} {
		if strings.Contains(rust, unwanted) {
			t.Fatalf("unexpected %q in:\n%s", unwanted, rust)
		}
	}
}

// The elements of a []interface{} are inside the slice. Spread into a print
// call they are its arguments.
func TestFormatAnySliceElementsAreNestedAndSpreadArgumentsAreNot(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Point struct{ X int }

func main() {
	items := []interface{}{&Point{1}, 2}
	fmt.Println(items)
	fmt.Println(items...)
}
`)

	for _, want := range []string{
		"format_value: fn(&dyn Any) -> String",
		"format!(\"[{}]\", format_any_slice_values(slice, format_any_nested))",
		"format_any_slice_values(slice, format_any)\n",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q in:\n%s", want, rust)
		}
	}

	plain := transpileTypedRegression(t, `package main

import "fmt"

func main() {
	items := []interface{}{1, 2}
	fmt.Println(items)
	fmt.Println(items...)
}
`)
	if !strings.Contains(plain, "format!(\"[{}]\", format_any_slice_values(slice))") {
		t.Fatalf("a program without a boxed pointer keeps the plain slice printer:\n%s", plain)
	}
	if strings.Contains(plain, "format_any_nested") {
		t.Fatalf("a program without a boxed pointer needs no nested printer:\n%s", plain)
	}
}

// Go prints a pointer through its String or Error method wherever the pointer
// sits, so such a pointer has no top-level and nested forms.
func TestBoxedPointerWithATextMethodPrintsThroughTheMethod(t *testing.T) {
	for name, method := range map[string]string{
		"value receiver String":   "func (s Status) String() string { return \"s\" }",
		"pointer receiver String": "func (s *Status) String() string { return \"s\" }",
		"value receiver Error":    "func (s Status) Error() string { return \"s\" }",
	} {
		t.Run(name, func(t *testing.T) {
			rust := transpileTypedRegression(t, `package main

import "fmt"

type Status struct{ Code int }

`+method+`

func show(v interface{}) { fmt.Println(v) }

func main() {
	show(&Status{1})
}
`)
			want := "value.downcast_ref::<Rc<RefCell<Option<Status>>>>() {\n        match (*v.borrow()).as_ref() { Some(__pointee) => format!(\"{}\", __pointee), None => \"<nil>\".to_string() }"
			if !strings.Contains(rust, want) {
				t.Fatalf("missing %q in:\n%s", want, rust)
			}
			if strings.Contains(rust, "go_format_any_pointer") {
				t.Fatalf("a pointer with a String or Error method is never printed as an address:\n%s", rust)
			}
		})
	}
}

// A value of a type with no formatting case cannot be printed. The program
// has to stop and say so; a placeholder would be a wrong answer that looks
// like a right one.
func TestFormatAnyStopsOnAValueItCannotPrint(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Row struct{ Value interface{} }

func main() {
	fmt.Println(Row{Value: 2})
}
`)

	if !strings.Contains(rust, "    } else {\n        panic!(\"go2rust: cannot print this value held in an interface: its type has no formatting case yet\")\n    }\n}") {
		t.Fatalf("format_any must stop on a type it has no case for:\n%s", rust)
	}
	if strings.Contains(rust, "<unknown>") {
		t.Fatalf("format_any must not print a placeholder:\n%s", rust)
	}
	for _, intType := range []string{"i8", "i16", "isize", "u8", "u16", "u32", "u64", "usize"} {
		if !strings.Contains(rust, "} else if let Some(v) = value.downcast_ref::<"+intType+">() {\n        v.to_string()") {
			t.Errorf("format_any has no case for %s:\n%s", intType, rust)
		}
	}
}

// Go prints a map sorted by key, false before true. Rust orders bool the
// same way, so a map with boolean keys gets a formatting case like a map with
// integer or string keys.
func TestBoxedMapWithBooleanKeysHasAFormattingCase(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func show(v interface{}) { fmt.Println(v) }

func main() {
	show(map[bool]int{true: 1})
}
`)

	if !strings.Contains(rust, "value.downcast_ref::<Rc<RefCell<Option<BTreeMap<bool, Rc<RefCell<Option<i64>>>>>>>>() {\n        format_map(v)") {
		t.Fatalf("a boxed map[bool]int has no formatting case:\n%s", rust)
	}
}

// A map of slices held in an interface gets a formatting case that prints
// each slice, and a nil one as [].
func TestBoxedMapOfSlicesHasAFormattingCase(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func show(v interface{}) { fmt.Println(v) }

func main() {
	show(map[string][]int{"a": nil})
}
`)
	if !strings.Contains(rust, "value.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<Vec<i64>>>>>>>>>() {\n        format_map_nil(v, \"[]\")") {
		t.Fatalf("a boxed map[string][]int has no formatting case:\n%s", rust)
	}
}

// A receiver's field used as a map key, ops[b.Op] inside a method of b, is
// looked up by the field's value, not by its cell.
func TestReceiverFieldMapKeyIsTheFieldValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Bin struct {
	Op   byte
	L, R float64
}

var ops = map[byte]func(a, b float64) float64{
	'+': func(a, b float64) float64 { return a + b },
}

func (b *Bin) Eval() float64 {
	return ops[b.Op](b.L, b.R)
}

func main() {
	b := &Bin{Op: '+', L: 1, R: 2}
	fmt.Println(b.Eval())
}
`)
	if strings.Contains(rust, ".get(&self.op.clone())") || strings.Contains(rust, ".get(&self.op)") {
		t.Fatalf("the map is looked up by the field's cell:\n%s", rust)
	}
	if !strings.Contains(rust, ".get(&{ let __selector_holder = self.op.clone();") {
		t.Fatalf("the map key should be the field's value:\n%s", rust)
	}
}

// A pointer with a String method held in one of the program's interfaces
// prints through String when the print goes through format_any.
func TestStringerPointerInProgramInterfacePrintsThroughString(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Node interface {
	Eval() float64
}

type Num struct{ V float64 }

func (n *Num) Eval() float64  { return n.V }
func (n *Num) String() string { return fmt.Sprint(n.V) }

func main() {
	var n Node = &Num{V: 2}
	fmt.Printf("%s => %g\n", n, 1.5)
}
`)
	want := `Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string()`
	if !strings.Contains(rust, want) {
		t.Fatalf("missing the pointee format arm %q:\n%s", want, rust)
	}
}
