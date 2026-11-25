package main

import (
	"strings"
	"testing"
)

func TestTailReturnStartingWithBlockIsParenthesized(t *testing.T) {
	src := `package main

import "fmt"

type byX []struct{ x int }

func (p byX) Less(i, j int) bool { return p[i].x < p[j].x }

func main() {
	p := byX{{2}, {1}}
	fmt.Println(p.Less(0, 1))
}
`
	rust := transpileTypedRegression(t, src)
	start := strings.Index(rust, "fn less(")
	if start < 0 {
		t.Fatalf("no less method:\n%s", rust)
	}
	body := rust[start:]
	body = body[strings.Index(body, "{")+1:]
	body = strings.TrimLeft(body, " \t\n")
	if strings.HasPrefix(body, "{") {
		t.Fatalf("tail expression starts with a block, which Rust parses as a statement:\n%s", rust[start:])
	}
}

func TestLocalInterfaceDeclaresTheMethodsOfAnEmbeddedStubInterface(t *testing.T) {
	src := `package main

import "fmt"

type Named interface {
	fmt.Stringer
	Name() string
}

type dog struct{ n string }

func (d dog) String() string { return "dog " + d.n }
func (d dog) Name() string   { return d.n }

func describe(x Named) string { return x.String() + "/" + x.Name() }

func main() {
	fmt.Println(describe(dog{"rex"}))
}
`
	rust := transpileTypedRegression(t, src)
	start := strings.Index(rust, "pub trait Named")
	if start < 0 {
		t.Fatalf("no Named trait:\n%s", rust)
	}
	trait := rust[start:]
	trait = trait[:strings.Index(trait, "\n}")]
	if !strings.Contains(trait, "fn string(&self)") || !strings.Contains(trait, "fn name(&self)") {
		t.Fatalf("Named does not declare fmt.Stringer's String:\n%s", trait)
	}
}

func TestTailValueNeedsParens(t *testing.T) {
	cases := map[string]bool{
		"x + 1":                          false,
		"{ let a = 1; a }":               false,
		"{ a } < { b }":                  true,
		"{ let c = '}'; c } == 'x'":      true,
		"{ let c = '\\''; c }":           false,
		"{ let s = \"}\"; s.len() }":     false,
		"{ let s = \"\\\"}\"; s }.len()": true,
		"{ let c = 'é'; c }":             false,
		"{ f::<'a>() }":                  false,
		"{ r#\"}\"# }":                   true,
	}
	for value, want := range cases {
		if got := tailValueNeedsParens(value); got != want {
			t.Errorf("tailValueNeedsParens(%q) = %v, want %v", value, got, want)
		}
	}
}
