package main

import (
	"strings"
	"testing"
)

// fmt.Fprintln to a type with its own Write method calls that method, as
// Fprint and Fprintf do. It printed to stdout.
func TestFprintlnToUserWriterCallsWrite(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Lines struct{ n int }

func (l *Lines) Write(p []byte) (int, error) {
	l.n += len(p)
	return len(p), nil
}

func main() {
	l := &Lines{}
	fmt.Fprintln(l, "a", 1)
	fmt.Println(l.n)
}
`)
	want := `{ let __s = format!("{} {}\n", `
	if !strings.Contains(rust, want) || !strings.Contains(rust, ".write(") {
		t.Errorf("Fprintln should format the line and call the writer's write:\n%s", rust)
	}
	if strings.Contains(rust, `println!("{} {}", format!("{}", "a".to_string())`) {
		t.Errorf("Fprintln to a writer should not print to stdout:\n%s", rust)
	}
}

// Fprintln(w, args...) and Fprint(w, args...) print the elements of args,
// not the slice.
func TestFprintSpreadPrintsEachElement(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import (
	"fmt"
	"os"
)

func main() {
	vals := []any{"x", 1}
	fmt.Fprintln(os.Stdout, vals...)
	fmt.Fprint(os.Stdout, vals...)
}
`)
	if !strings.Contains(rust, `print!("{}", format!("{}\n", format_any_variadic(&`) {
		t.Errorf("Fprintln with a spread should print each element:\n%s", rust)
	}
	if !strings.Contains(rust, `print!("{}", { let __sprint_values = `) {
		t.Errorf("Fprint with a spread should print each element:\n%s", rust)
	}
}

// A field of the method's receiver goes into a []any as its value. The
// field's cell was boxed, and printing it panicked.
func TestAnySliceLiteralBoxesReceiverFieldValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type L struct{ prefix string }

func (l L) Args() []any { return []any{l.prefix} }

func main() { fmt.Println(L{"p"}.Args()...) }
`)
	if strings.Contains(rust, "Box::new(self.prefix.clone()) as Box<dyn Any>") {
		t.Errorf("the receiver field's cell should not be boxed:\n%s", rust)
	}
	if !strings.Contains(rust, "Box::new({ let __selector_holder = self.prefix.clone();") {
		t.Errorf("missing the boxed field value:\n%s", rust)
	}
}
