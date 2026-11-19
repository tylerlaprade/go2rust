package main

import (
	"strings"
	"testing"
)

// fmt prints <nil> for a nil error. The print site must not unwrap the
// handle, which panics when the error is nil.
func TestPrintingAnErrorPrintsNilAsNil(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func find() error { return nil }

func main() {
	err := find()
	fmt.Println(err)
	fmt.Printf("%v\n", err)
	fmt.Println(find())
}
`)

	want := `{ let __text = match err.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }`
	if got := strings.Count(rust, want); got != 2 {
		t.Fatalf("want the nil-safe print of err twice, got %d in:\n%s", got, rust)
	}
	if !strings.Contains(rust, `{ let __text = match find().borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }`) {
		t.Fatalf("a call that returns an error must print nil-safely too:\n%s", rust)
	}
	if strings.Contains(rust, "err.borrow().as_ref().unwrap()") {
		t.Fatalf("printing must not unwrap an error that may be nil:\n%s", rust)
	}
}

// fmt prints a pointer to a struct as & and the struct, a nil one as <nil>,
// and one whose type has a String or Error method through that method.
func TestPrintingAPointerFollowsItsMethodSet(t *testing.T) {
	for name, test := range map[string]struct {
		method string
		format string
	}{
		"no method":               {"", "&{}"},
		"value receiver String":   {"func (p Point) String() string { return \"p\" }", "{}"},
		"pointer receiver String": {"func (p *Point) String() string { return \"p\" }", "{}"},
		"pointer receiver Error":  {"func (p *Point) Error() string { return \"p\" }", "{}"},
		"unrelated method":        {"func (p *Point) Name() string { return \"p\" }", "&{}"},
		"String with a parameter": {"func (p *Point) String(prefix string) string { return prefix }", "&{}"},
	} {
		t.Run(name, func(t *testing.T) {
			rust := transpileTypedRegression(t, `package main

import "fmt"

type Point struct{ X int }

`+test.method+`

func main() {
	p := &Point{1}
	fmt.Println(p)
}
`)
			want := `{ let __text = match p.borrow().as_ref() { Some(__value) => format!("` + test.format + `", __value), None => "<nil>".to_string() }; __text }`
			if !strings.Contains(rust, want) {
				t.Fatalf("missing %q in:\n%s", want, rust)
			}
		})
	}
}

// The handle is released before the next argument is evaluated, so the same
// locked error can be printed twice in one call.
func TestPrintingALockedErrorTwiceReleasesItBetweenArguments(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

import "fmt"

func find() error { return nil }

func main() {
	done := make(chan bool, 1)
	go func() { done <- true }()
	<-done
	err := find()
	fmt.Printf("%v %s\n", err, err)
}
`)

	want := `{ let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }`
	if got := strings.Count(rust, want); got != 2 {
		t.Fatalf("want two self-contained prints of err, got %d in:\n%s", got, rust)
	}
}

// "var err error = value" boxes a struct or pointer that implements error,
// as an assignment to an error variable does.
func TestErrorVariableDeclaredFromAConcreteErrorIsBoxed(t *testing.T) {
	for name, source := range map[string]string{
		"pointer receiver": `package main

type Broken struct{ Why string }

func (b *Broken) Error() string { return b.Why }

func main() {
	var err error = &Broken{"y"}
	_ = err
}
`,
		"value receiver": `package main

type Broken struct{ Why string }

func (b Broken) Error() string { return b.Why }

func main() {
	var err error = Broken{"y"}
	_ = err
}
`,
	} {
		t.Run(name, func(t *testing.T) {
			rust := transpileTypedRegression(t, source)
			want := "let mut err: Rc<RefCell<Option<Box<dyn StdError>>>> = Rc::new(RefCell::new(Some(Box::new(Broken {"
			if !strings.Contains(rust, want) {
				t.Fatalf("missing %q in:\n%s", want, rust)
			}
			if !strings.Contains(rust, ") as Box<dyn StdError>)));") {
				t.Fatalf("the value must be boxed as an error:\n%s", rust)
			}
		})
	}
}
