package main

import (
	"strings"
	"testing"
)

// A struct with a pointer field compares the pointers, not what they point
// to, so it gets its own PartialEq and Ord instead of derived ones (#354).
func TestStructPointerFieldComparesPointers(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type K struct{ P *int }

func main() {
	a, b := 1, 1
	m := map[K]int{}
	m[K{&a}]++
	m[K{&b}]++
	fmt.Println(len(m), K{&a} == K{&b})
}
`)
	for _, want := range []string{"impl PartialEq for K {", "Rc::ptr_eq(&self.p, &other.p)", "impl Ord for K {", "Rc::as_ptr(&self.p) as usize"} {
		if !strings.Contains(rust, want) {
			t.Fatalf("missing %s:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "#[derive(Debug, Clone, Default, PartialEq") {
		t.Fatalf("K should not derive PartialEq:\n%s", rust)
	}
}
