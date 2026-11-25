package main

import (
	"strings"
	"testing"
)

// A *T held in a local interface prints through __go_print, which the
// pointer wrapper overrides to print &{...} or the pointee's String()
// (DEV-170).
func TestLocalInterfacePointerPrintsPointee(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Shape interface{ Area() int }

type Rect struct{ w, h int }

func (r *Rect) Area() int { return r.w * r.h }

type Square struct{ side int }

func (s *Square) Area() int      { return s.side * s.side }
func (s *Square) String() string { return "square" }

func main() {
	var s Shape = &Rect{1, 2}
	fmt.Println(s)
	fmt.Printf("%+v\n", s)
	var q Shape = &Square{3}
	fmt.Println(q)
}
`)
	for _, want := range []string{
		// The trait's default prints the value's Display.
		`fn __go_print(&self, plus: bool) -> String { if plus { format!("{:#}", self) } else { format!("{}", self) } }`,
		// A pointer to a struct without String() prints &{...}.
		`Some(__v) => if plus { format!("&{:#}", __v) } else { format!("&{}", __v) }, None => "<nil>".to_string() }`,
		// A pointer whose type has String() prints the pointee's Display, which calls it.
		`Some(__v) => format!("{}", __v), None => "<nil>".to_string() }`,
		// Print sites ask the interface value to print itself.
		`Some(__iface) => __iface.as_ref().__go_print(false)`,
		`Some(__iface) => __iface.as_ref().__go_print(true)`,
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q in:\n%s", want, rust)
		}
	}
	// The wrapper's Display stays the address, for nested prints and map keys.
	if !strings.Contains(rust, `write!(f, "{:p}", __v as *const _)`) {
		t.Errorf("the pointer wrapper's Display should still print the address:\n%s", rust)
	}
}

// An interface that embeds two interfaces inherits __go_print from both, so
// the print site names one of them (DEV-170).
func TestEmbeddingInterfacePrintNamesDeclaringTrait(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

import "fmt"

type Named interface{ Name() string }
type Sized interface{ Size() int }
type Item interface {
	Named
	Sized
}

type File struct{ name string }

func (f *File) Name() string { return f.name }
func (f *File) Size() int     { return 1 }

func main() {
	var it Item = &File{"a"}
	fmt.Println(it)
}
`)
	if !strings.Contains(rust, "Named::__go_print(__iface.as_ref(), false)") {
		t.Errorf("the print should name the declaring trait:\n%s", rust)
	}
}
