package main

import (
	"strings"
	"testing"
)

// g(f()) with a multi-value f binds f's results first, then passes them to g.
func TestMultiValueCallSuppliesAllArguments(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

func divmod(a, b int) (int, int) { return a / b, a % b }

func main() {
	fmt.Println(divmod(7, 3))
}
`)
	if !strings.Contains(rust, "let (mut __go_mv_") || !strings.Contains(rust, ") = divmod(") {
		t.Errorf("divmod's results should be bound before the call:\n%s", rust)
	}
	if strings.Contains(rust, "divmod(Rc::new(RefCell::new(Some(7))), Rc::new(RefCell::new(Some(3)))).borrow()") {
		t.Errorf("the tuple is still read as one wrapped value:\n%s", rust)
	}
}
