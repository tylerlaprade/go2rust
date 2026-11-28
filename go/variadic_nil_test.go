package main

import (
	"strings"
	"testing"
)

// A variadic call with no variadic arguments passes a nil slice, as Go's
// spec says, so the callee sees xs == nil (#369).
func TestVariadicCallWithoutArgumentsPassesNil(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func f(xs ...int) bool { return xs == nil }

func g(p string, xs ...int) bool { return xs == nil }

func main() {
	fmt.Println(f(), g("p"))
}
`)
	for _, want := range []string{"f(Rc::new(RefCell::new(None)))", `g(Rc::new(RefCell::new(Some("p".to_string()))), Rc::new(RefCell::new(None)))`} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %s:\n%s", want, rust)
		}
	}
}
