package main

import (
	"strings"
	"testing"
)

func TestDefinedArrayOfArraysWritesComparesAndPrints(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Rows [2][2]int
type Row [3]int

func (r *Rows) Set(i, j, v int) { r[i][j] = v }

func (r Row) First() int { return r[0] }

func main() {
	var r Rows
	r[1][0] = 5
	r.Set(0, 1, 2)
	fmt.Println(r, r == Rows{}, Row{1, 2, 3}.First())
}
`)
	for _, want := range []string{
		// The nested write goes through the array inside the newtype.
		"let __seq = { let __named_array = (*r.borrow().as_ref().unwrap()).0.clone(); __named_array }; let __outer_idx = (1) as usize; let __idx = (0) as usize; let __rhs = 5; (*__seq.borrow_mut().as_mut().unwrap())[__outer_idx][__idx] = __rhs;",
		"let __seq = self.0.clone(); let __outer_idx = ",
		"impl PartialEq for Rows {",
		"impl Display for Rows {\n    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {\n        write!(f, \"{}\", format_nested_slice(&self.0))",
		// An element read is a block, so it cannot be the body's tail.
		"pub fn first(&self) -> i64 {\n        return { let __seq_holder = self.0.clone();",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q in:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "format_slice(&r)") {
		t.Errorf("a defined array of arrays should print through its Display:\n%s", rust)
	}
}

func TestMapLiteralKeepsCallResultCell(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type P struct{ X int }

func mk() P { return P{1} }
func pair() [2]int { return [2]int{1, 2} }

func main() {
	m := map[string]P{"p": mk()}
	a := map[string][2]int{"a": pair()}
	_, _ = m, a
}
`)
	for _, want := range []string{
		`("p".to_string(), mk())`,
		`("a".to_string(), pair())`,
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q in:\n%s", want, rust)
		}
	}
}

func TestStringVariableIsCopiedIntoElement(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

func set(s []string, b *[2][2]string, name string) {
	s[0] = name
	b[1][0] = name
}

func main() {
	var b [2][2]string
	set([]string{"a"}, &b, "z")
}
`)
	for _, want := range []string{
		"let __idx = (0) as usize; let __rhs = (*name.borrow().as_ref().unwrap()).clone(); (*s.borrow_mut().as_mut().unwrap())[__idx] = __rhs;",
		"let __outer_idx = (1) as usize; let __idx = (0) as usize; let __rhs = (*name.borrow().as_ref().unwrap()).clone(); (*b.borrow_mut().as_mut().unwrap())[__outer_idx][__idx] = __rhs;",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q in:\n%s", want, rust)
		}
	}
}
