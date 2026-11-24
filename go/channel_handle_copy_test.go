package main

import (
	"strings"
	"testing"
)

// A channel stored in a struct literal, a slice or map literal, or sent on
// a channel of channels is a clone of the handle, so the original variable
// can still be used (DEV-144).
func TestChannelStoredInLiteralsIsAHandleClone(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type holder struct{ ch chan int }

func main() {
	c := make(chan int, 4)
	h := holder{ch: c}
	h.ch <- 1
	chans := []chan int{c}
	byName := map[string]chan int{"c": c}
	cc := make(chan chan int, 1)
	cc <- c
	byName["c"] <- 2
	fmt.Println(<-c, len(chans), <-cc == nil)
}
`)
	for _, want := range []string{
		"holder { ch: c.clone()",
		"vec![c.clone()]",
		"Some(c.clone())",
		"cc.send(c.clone())",
		"let __channel = __channel_cell",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q:\n%s", want, rust)
		}
	}
	for _, moved := range []string{"holder { ch: c,", "vec![c]", "Some(c))", "cc.send(c)"} {
		if strings.Contains(rust, moved) {
			t.Errorf("the channel is still moved, found %q:\n%s", moved, rust)
		}
	}
}

// A comma-ok receive from a struct field reads the channel handle, and a
// blank name is Rust's wildcard.
func TestCommaOkReceiveFromFieldWithBlankValue(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type holder struct{ ch chan int }

func main() {
	h := holder{ch: make(chan int)}
	close(h.ch)
	_, ok := <-h.ch
	var v int
	v, ok = <-h.ch
	_, ok = <-h.ch
	fmt.Println(v, ok)
}
`)
	if strings.Contains(rust, "mut _") {
		t.Errorf("a blank name should be a wildcard without mut:\n%s", rust)
	}
	if !strings.Contains(rust, "let (_, mut ok) = match { let __channel = ") {
		t.Errorf("the receive should read the field's channel handle:\n%s", rust)
	}
	if !strings.Contains(rust, ".recv() { Some(_) => {") {
		t.Errorf("a blank value target should discard the value:\n%s", rust)
	}
}
