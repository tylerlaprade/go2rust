package main

import (
	"strings"
	"testing"
)

const fmtStringerRegressionSource = `package main

import "fmt"

type Celsius struct{ Deg int }

func (c Celsius) String() string { return fmt.Sprintf("%dC", c.Deg) }

type Counter struct{ N int }

func (c *Counter) String() string { return fmt.Sprintf("Counter<%d>", c.N) }

func describe(v any) string {
	switch x := v.(type) {
	case fmt.Stringer:
		return x.String()
	}
	return "other"
}

func label(v any) string {
	if s, ok := v.(fmt.Stringer); ok {
		return s.String()
	}
	return "none"
}

func main() {
	var s fmt.Stringer = Celsius{21}
	var p fmt.Stringer = &Counter{N: 1}
	var v any = Celsius{2}
	fmt.Println(s, p.String(), describe(v), label(v), v.(fmt.Stringer).String())
}
`

// A value converted to fmt.Stringer carries a closure over its Display
// impl, which calls its String method (DEV-152).
func TestFmtStringerConversionCarriesValueText(t *testing.T) {
	rust := transpileTypedRegression(t, fmtStringerRegressionSource)
	for _, want := range []string{
		"__go_text: Option<Rc<dyn Fn() -> String>>",
		"fmt_Stringer::__go_from_display(Celsius {",
		"fmt_Stringer::__go_from_display_cell(",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q:\n%s", want, rust)
		}
	}
}

// case fmt.Stringer and x.(fmt.Stringer) on an any match the program's types
// with a String method, not only a value that is already a fmt_Stringer.
func TestFmtStringerSwitchAndAssertionMatchStringMethodTypes(t *testing.T) {
	rust := transpileTypedRegression(t, fmtStringerRegressionSource)
	for _, want := range []string{
		"_ts_val.and_then(|__v| __v.downcast_ref::<fmt_Stringer>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<",
		"else if let Some(typed_val) = any_val.downcast_ref::<Celsius>() { (Rc::new(RefCell::new(Some(fmt_Stringer::__go_from_display(typed_val.clone())))), true) }",
		"else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Counter>>>>() { Ok(fmt_Stringer::__go_from_display_cell(typed_val.clone())) }",
		// A value receiver's String makes *Celsius a Stringer too.
		"else if let Some(typed_val) = any_val.downcast_ref::<Rc<RefCell<Option<Celsius>>>>() { Ok(fmt_Stringer::__go_from_display_cell(typed_val.clone())) }",
		`Err(format!("interface conversion: {} is not fmt.Stringer: missing method String", __go_type_name(&**any_val)))`,
		`Err("interface conversion: interface is nil, not fmt.Stringer".to_string())`,
		`match __stringer { Ok(__s) => __s, Err(__message) => panic!("{}", __message) }`,
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q:\n%s", want, rust)
		}
	}
}

// x.(fmt.Stringer) yields the bare fmt_Stringer, so a method called on it
// directly is not routed through a cell borrow.
func TestFmtStringerAssertionMethodReceiverIsBare(t *testing.T) {
	rust := transpileTypedRegression(t, fmtStringerRegressionSource)
	if !strings.Contains(rust, "match __stringer { Ok(__s) => __s, Err(__message) => panic!(\"{}\", __message) }\n    }).string()") {
		t.Fatalf("String should be called on the asserted value itself:\n%s", rust)
	}
}

// s, ok := any(c).(fmt.Stringer) goes to the fmt.Stringer writer, not the
// shortcut that hands back the operand, and %T of a fmt.Stringer names the
// type of the value it holds.
func TestFmtStringerInPlaceAssertionAndTypeName(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Celsius struct{ Deg int }

func (c Celsius) String() string { return fmt.Sprint(c.Deg) }

func main() {
	if st, ok := any(Celsius{1}).(fmt.Stringer); ok {
		fmt.Printf("%T %v\n", st, st)
	}
}
`)
	for _, want := range []string{
		"else if let Some(typed_val) = any_val.downcast_ref::<Celsius>() { (Rc::new(RefCell::new(Some(fmt_Stringer::__go_from_display(typed_val.clone())))), true) }",
		`match (st).borrow().as_ref() { Some(__stringer) => __go_type_name(&*__stringer.__go_value), None => "<nil>" }`,
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q:\n%s", want, rust)
		}
	}
}

// A call's result converts to fmt.Stringer by its value, and an assertion
// to fmt.Stringer prints and names its held type without the cell steps of
// a variable.
func TestFmtStringerCallConversionAndAssertionArguments(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Celsius struct{ Deg int }

func (c Celsius) String() string { return fmt.Sprint(c.Deg) }

func makeC() Celsius { return Celsius{3} }

func main() {
	var s fmt.Stringer = makeC()
	var v any = Celsius{2}
	fmt.Println(s, v.(fmt.Stringer))
	fmt.Printf("%T\n", v.(fmt.Stringer))
}
`)
	for _, want := range []string{
		"fmt_Stringer::__go_from_display((*make_c().borrow().as_ref().unwrap()).clone())",
		"__go_type_name(&*(({",
		").__go_value)",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q:\n%s", want, rust)
		}
	}
}

// A fmt.Stringer put into an interface{} is boxed as the value it holds, so
// an assertion to that value's type finds it; a nil one is a nil interface
// (DEV-155).
func TestFmtStringerBoxesItsValueIntoAny(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type C struct{ v int }

func (c C) String() string { return "c" }

func main() {
	var s fmt.Stringer = C{1}
	var y any = s
	list := []any{s}
	c, ok := y.(C)
	fmt.Println(c, ok, len(list), any(s) != nil)
}
`)
	for _, want := range []string{
		"pub fn __go_into_any(&self) -> Box<dyn std::any::Any>",
		"let __into_any: Rc<dyn Fn() -> Box<dyn std::any::Any>> = Rc::new(move || Box::new((*boxed).clone()) as Box<dyn std::any::Any>);",
		"match (*s.borrow()).as_ref() { Some(__stringer) => __stringer.__go_into_any(), None => Box::new(GoNilAny) as Box<dyn Any> }",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "Box::new((*s.borrow().as_ref().unwrap()).clone()) as Box<dyn Any>") {
		t.Fatalf("a fmt.Stringer should not be boxed as the fmt_Stringer itself:\n%s", rust)
	}
}
