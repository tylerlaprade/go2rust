package main

import (
	"strings"
	"testing"
)

const errorCopyBlock = "{ let __err_source = %s.clone(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }"

// Go copies an error value freely, and each copy is a variable of its own
// that holds the same error. A Box cannot be cloned, so the generated code
// used to move the error out of the variable it was assigned from, or to
// make both names one variable.
func TestErrorIsHeldByASharedPointer(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import (
	"errors"
	"fmt"
)

type Missing struct{ Name string }

func (m Missing) Error() string { return "missing " + m.Name }

func find(name string) error {
	if name == "" {
		return Missing{name}
	}
	return fmt.Errorf("no %s", name)
}

func main() {
	var none error
	made := errors.New("made")
	fmt.Println(none, made, find(""))
}
`)

	for _, want := range []string{
		"let mut none: Rc<RefCell<Option<Rc<dyn StdError>>>> = Rc::new(RefCell::new(None));",
		`Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("made".to_string())))))`,
		"Rc::new(Missing {",
		" as Rc<dyn StdError>",
		"pub fn find(name: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<Rc<dyn StdError>>>> {",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q in:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "Box<dyn StdError>>") {
		t.Fatalf("an error should not be held by a Box:\n%s", rust)
	}
}

// Standalone transpilation must allocate pointer-error wrapper names with the
// full package set in view. Otherwise E and E_1 both choose
// __go_error_ptr_E_1 when the source already declares __go_error_ptr_E.
func TestStandalonePointerErrorWrapperNamesDoNotCollide(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type E struct{}
func (*E) Error() string { return "e" }

type E_1 struct{}
func (*E_1) Error() string { return "e1" }

type __go_error_ptr_E struct{}

func first() error { return &E{} }
func second() error { return &E_1{} }
`)

	if got := strings.Count(rust, "struct __go_error_ptr_E_1("); got != 1 {
		t.Fatalf("generated pointer-error wrapper name must be unique, got %d definitions:\n%s", got, rust)
	}
	if !strings.Contains(rust, "struct __go_error_ptr_E_1_1(") {
		t.Fatalf("second pointer-error wrapper should advance past the allocated name:\n%s", rust)
	}
}

func TestPointerErrorDisplayDispatchesTypedNilReceiver(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Safe struct{}
func (*Safe) Error() string { return "custom" }

func main() {
	var p *Safe
	var err error = p
	fmt.Println(err)
}
`)

	for _, want := range []string{
		"go_silent_catch_unwind(|| __go_nil_recv_safe_error(Rc::new(RefCell::new(None))))",
		"Err(_) => write!(f, \"<nil>\")",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("typed-nil pointer error formatting missing %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "let __go_previous_panic_hook") {
		t.Fatalf("typed-nil formatting should not swap the process-wide panic hook per call:\n%s", rust)
	}
}

// "last = err" leaves err as it was. It used to empty it.
func TestAssigningAnErrorLeavesTheSource(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "errors"

func run() error { return errors.New("boom") }

func main() {
	var last error
	err := run()
	if err != nil {
		last = err
	}
	_ = last
}
`)

	want := "{ let __rhs_holder = err.clone(); let new_val = (*__rhs_holder.borrow()).clone(); *last.borrow_mut() = new_val; }"
	if !strings.Contains(rust, want) {
		t.Fatalf("missing %q in:\n%s", want, rust)
	}
	if strings.Contains(rust, ".take()") {
		t.Fatalf("assigning an error must not take it out of the variable it comes from:\n%s", rust)
	}
}

// "e2 := e1" makes a second variable. It used to make a second name for the
// first one, so that assigning to e1 changed e2. The same holds for an error
// that is passed to a function, returned from a field, or put in a struct.
func TestAnErrorVariableIsCopiedIntoItsOwnHandle(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "errors"

type Reader struct{ err error }

func (r *Reader) Err() error { return r.err }

func report(err error) {}

func pass(err error) error { return err }

func main() {
	e1 := errors.New("first")
	e2 := e1
	e1 = errors.New("second")
	report(e2)
	r := Reader{err: e2}
	_ = r.Err()
	_ = pass(e1)
}
`)

	for _, want := range []string{
		"let mut e2 = " + strings.Replace(errorCopyBlock, "%s", "e1", 1) + ";",
		"report(" + strings.Replace(errorCopyBlock, "%s", "e2", 1) + ");",
		"err: " + strings.Replace(errorCopyBlock, "%s", "e2", 1),
		// A method that returns an error field returns a copy, not the field.
		"let __err_source = { let __field = self.err.clone(); __field };",
		// So does a function that returns an error variable.
		"-> Rc<RefCell<Option<Rc<dyn StdError>>>> {\n    " + strings.Replace(errorCopyBlock, "%s", "err", 1) + "\n}",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q in:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "let mut e2 = e1.clone();") {
		t.Fatalf("e2 must not share the handle of e1:\n%s", rust)
	}
}

// Two errors.New values are equal only when both interface values retain the
// same dynamic error allocation. Two variables holding one error have two
// outer handles, so comparing those handles is not enough.
func TestErrorsAreComparedByIdentity(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import (
	"errors"
	"fmt"
)

func main() {
	a := errors.New("same")
	b := errors.New("same")
	c := a
	fmt.Println(a == b, a == c)
}
`)

	for _, want := range []string{
		"match (__left_value.as_ref(), __right_value.as_ref())",
		"std::ptr::addr_eq(&**__l, &**__r)",
		"(None, None) => true",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q in:\n%s", want, rust)
		}
	}
}

// In a program with goroutines the pointer is an Arc and the error is Send
// and Sync.
func TestErrorIsHeldByAnArcInAProgramWithGoroutines(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

import (
	"errors"
	"fmt"
)

func main() {
	done := make(chan bool)
	go func() { done <- true }()
	<-done
	var last error
	err := errors.New("boom")
	last = err
	copied := err
	fmt.Println(last, copied, err == copied)
}
`)

	for _, want := range []string{
		"let mut last: Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));",
		`Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("boom".to_string()))`,
		"{ let __rhs_holder = err.clone(); let new_val = (*__rhs_holder.lock().unwrap()).clone(); *last.lock().unwrap() = new_val; }",
		"let mut copied = { let __err_source = err.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) };",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q in:\n%s", want, rust)
		}
	}
}

// The helpers that are written out as Rust text get the representation
// filled in, with the value of GO_ERROR_FROM found by matching parentheses.
func TestExpandErrorReprFillsHelperText(t *testing.T) {
	text := `fn f() -> Option<GO_ERROR_PAYLOAD> { Some(GO_ERROR_FROM(std::io::Error::new(kind(), "x"))) } // GO_ERROR_FROM(e)`
	got := expandErrorRepr(text, false)
	want := `fn f() -> Option<std::rc::Rc<dyn std::error::Error>> { Some(std::rc::Rc::<dyn std::error::Error>::from(Box::<dyn std::error::Error>::from(std::io::Error::new(kind(), "x")))) } // std::rc::Rc::<dyn std::error::Error>::from(Box::<dyn std::error::Error>::from(e))`
	if got != want {
		t.Fatalf("sequential:\n got %s\nwant %s", got, want)
	}
	got = expandErrorRepr("GO_ERROR_PAYLOAD GO_ERROR_FROM(e)", true)
	want = "std::sync::Arc<dyn std::error::Error + Send + Sync> std::sync::Arc::<dyn std::error::Error + Send + Sync>::from(Box::<dyn std::error::Error + Send + Sync>::from(e))"
	if got != want {
		t.Fatalf("concurrent:\n got %s\nwant %s", got, want)
	}
}

// An error of a type the program declares goes into an interface{} as
// itself, where format_any and go_any_clone find it. It used to go in inside
// a wrapper that no reader opened, so it printed as "<unknown>" and stopped
// the program when the interface value was copied.
func TestCustomErrorIsBoxedIntoAnInterfaceAsItself(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Missing struct{ Name string }

func (m Missing) Error() string { return "missing " + m.Name }

type NotFound struct{ Code int }

func (n *NotFound) Error() string { return "not found" }

func find() error { return Missing{"key"} }

func lookup() error { return &NotFound{Code: 404} }

func log(v interface{}) { fmt.Println(v) }

func main() {
	log(find())
	log(lookup())
}
`)

	for _, want := range []string{
		// A struct error is boxed as the struct.
		`if let Some(typed_val) = __err.downcast_ref::<Missing>() { go_box_any_with_metadata(typed_val.clone(), "struct", true) }`,
		// A pointer error is boxed as a pointer handle, as any pointer is.
		`if let Some(typed_val) = __err.downcast_ref::<__go_error_ptr_NotFound>() { go_box_any_with_metadata(typed_val.0.clone(), "pointer", true) }`,
		// The box holds the value and nothing else.
		"go_register_any_type::<T>(kind, comparable);\n    Box::new(value) as Box<dyn Any>",
		// Both types are known to the readers of an interface value.
		"} else if let Some(v) = value.downcast_ref::<Missing>() {",
		"} else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<NotFound>>>>() {",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %q in:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "GoAnyMetadataBox") {
		t.Fatalf("no wrapper should stand between an interface value and what it holds:\n%s", rust)
	}
}

// An error taken back out of an interface{} is a copy, as every other use of
// an error value is. All three forms give the result a handle of its own to
// the same error: "got = nil" afterwards must not empty the interface value.
// The one-value form also has to find an error that was boxed as a handle,
// which is how the value of errors.New is boxed; it used to look only for a
// string and stopped the program.
func TestAnErrorTakenOutOfAnInterfaceIsACopy(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import (
	"errors"
	"fmt"
)

func main() {
	var boxed any = errors.New("x")
	got, ok := boxed.(error)
	got = nil
	single := boxed.(error)
	single = nil
	switch v := boxed.(type) {
	case error:
		v = nil
		_ = v
	}
	fmt.Println(got, ok, single)
}
`)

	const handle = "Rc<RefCell<Option<Rc<dyn StdError>>>>"
	for what, want := range map[string]string{
		"the two-value form": "if let Some(typed_val) = any_val.downcast_ref::<" + handle + ">() {\n                ({ let __err_source = typed_val; let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) }, true)",
		"the one-value form": "{ if let Some(__err_handle) = any_val.downcast_ref::<" + handle + ">() { (*__err_handle.borrow()).clone().expect(\"type assertion failed\") } else { let typed_val = any_val.downcast_ref::<std::string::String>().unwrap_or_else(|| panic!(\"interface conversion: {} is {}, not {}\", \"interface {}\", __go_type_name(&**any_val), \"error\"));",
		"case error":         "let mut v = { let __err_source = _ts_val.and_then(|__v| __v.downcast_ref::<" + handle + ">()).unwrap(); let __err_copy = (*__err_source.borrow()).clone(); Rc::new(RefCell::new(__err_copy)) };",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("%s: missing %q", what, want)
		}
	}
	if t.Failed() {
		t.Logf("generated Rust:\n%s", rust)
	}
	if strings.Contains(rust, "(typed_val.clone(), true)") {
		t.Errorf("an asserted error must not share the interface value's own handle:\n%s", rust)
	}
}
