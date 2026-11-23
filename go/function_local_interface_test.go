package main

import (
	"strings"
	"testing"
)

func TestFunctionLocalInterfaceGetsTraitImpls(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type animal struct{ name string }

func (a animal) Name() string { return a.name }

type dog struct {
	animal
}

func (d dog) Sound() string { return "woof" }

type counter struct{ n int }

func (c *counter) Inc() { c.n++ }

func main() {
	type speaker interface {
		Name() string
		Sound() string
	}
	var s speaker = dog{animal{"rex"}}
	_ = s.Sound()

	type incer interface{ Inc() }
	var i incer = &counter{}
	i.Inc()
}
`)

	for _, want := range []string{
		"impl speaker for dog {",
		"impl incer for counterPtr {",
		"fn inc(&mut self);",
	} {
		if !strings.Contains(rust, want) {
			t.Fatalf("an interface declared in a function needs its trait impls, missing %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "impl speaker for animal {") {
		t.Fatalf("animal lacks Sound and must not implement speaker:\n%s", rust)
	}
}
