package main

import (
	"strings"
	"testing"
)

// A channel of funcs carries the Option of a func value, so nil can be
// sent and a receive from a closed channel gives nil.
func TestFuncChannelCarriesTheFuncValueOption(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func main() {
	ch := make(chan func() int, 2)
	ch <- func() int { return 1 }
	ch <- nil
	f := <-ch
	_ = f
}
`)
	if !strings.Contains(rust, "GoChannel::<Option<") {
		t.Fatalf("a chan func() int should carry Option<...> values:\n%s", rust)
	}
	if !strings.Contains(rust, ".send(None::<") {
		t.Fatalf("sending nil should send None:\n%s", rust)
	}
	if !strings.Contains(rust, ".recv().unwrap_or_default()))") {
		t.Fatalf("a receive should fill a cell of its own:\n%s", rust)
	}
}
