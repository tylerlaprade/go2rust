package main

import (
	"strings"
	"testing"
)

// %T of an error names the error it holds, matched against the package's
// error types, not the type of its text (DEV-186).
func TestErrorTypeNameMatchesPackageErrorTypes(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type NotFound struct{ Key string }

func (e NotFound) Error() string { return e.Key }

type Limit struct{ Max int }

func (e *Limit) Error() string { return "limit" }

func main() {
	var err error = NotFound{"k"}
	fmt.Printf("%T\n", err)
}
`)
	for _, want := range []string{
		`if __e.downcast_ref::<Limit>().is_some() { "*main.Limit" } else if __e.downcast_ref::<NotFound>().is_some() { "main.NotFound" } else `,
		`if __wrap.many { "*errors.joinError" } else { "*fmt.wrapError" } } else { "*errors.errorString" }`,
		`None => "<nil>"`,
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q in:\n%s", want, rust)
		}
	}
}
