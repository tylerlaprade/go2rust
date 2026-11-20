package main

import (
	"strings"
	"testing"
)

func TestScalarNewtypeCloneCopiesTheValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Counter int

func (c *Counter) Inc() { *c++ }

func main() {
	a := Counter(1)
	b := a
	b.Inc()
	fmt.Println(a, b)
	for _, c := range []Counter{1, 2} {
		c.Inc()
	}
}
`)
	if !strings.Contains(rust, "impl Clone for Counter {") || !strings.Contains(rust, "Counter(Rc::new(RefCell::new(__value)))") {
		t.Fatalf("expected a Clone that copies the value into its own handle:\n%s", rust)
	}
	if strings.Contains(rust, "#[derive(Debug, Clone, Default)]\npub struct Counter") {
		t.Fatalf("the derived Clone shares the handle:\n%s", rust)
	}
	if !strings.Contains(rust, "for mut c in") {
		t.Fatalf("a pointer method on the range value needs it mutable:\n%s", rust)
	}
}
