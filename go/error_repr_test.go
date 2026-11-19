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

// Two errors are equal when they are the same error: both nil, or both
// pointers to one allocation. Two variables holding one error have two
// handles, so comparing the handles is not enough.
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

	want := "match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__l), Some(__r)) => std::ptr::addr_eq(&**__l, &**__r), (None, None) => true, _ => false }"
	if !strings.Contains(rust, want) {
		t.Fatalf("missing %q in:\n%s", want, rust)
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
