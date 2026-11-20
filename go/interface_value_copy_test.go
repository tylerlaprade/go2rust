package main

import (
	"strings"
	"testing"
)

const twoEmbeddedInterfacesSource = `package main

type Shape interface{ Area() float64 }
type Named interface{ Name() string }
type NamedShape interface {
	Shape
	Named
}

type Rect struct{ W, H float64 }

func (r Rect) Area() float64 { return r.W * r.H }
func (r Rect) Name() string  { return "rect" }

func main() {
	var a, b NamedShape = Rect{1, 2}, Rect{1, 2}
	_ = a == b
	if r, ok := a.(Rect); ok {
		_ = r
	}
	switch a.(type) {
	case Rect:
	}
	var plain []Shape
	plain = append(plain, a)
	_ = plain
}
`

func TestInterfaceEmbeddingTwoInterfacesNamesTheDeclaringTrait(t *testing.T) {
	rust := transpileTypedRegression(t, twoEmbeddedInterfacesSource)
	// Shape and Named both declare __go_as_any, so method syntax on a
	// NamedShape trait object is ambiguous (E0034).
	for _, want := range []string{
		"Shape::__go_as_any(&**self)",
		"Shape::__go_as_any(other).downcast_ref::<Rect>()",
		"Shape::__go_as_any(any_val.as_ref())",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("missing %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "(**self).__go_as_any()") {
		t.Errorf("a NamedShape adapter still calls __go_as_any by method syntax:\n%s", rust)
	}
	// A NamedShape appended to a []Shape is converted to a Shape.
	if !strings.Contains(rust, "-> Box<dyn Shape> { __v }") {
		t.Errorf("appending a NamedShape to []Shape should convert it:\n%s", rust)
	}
}

func TestInterfaceEmbeddingOneInterfaceKeepsMethodSyntax(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Shape interface{ Area() float64 }
type Solid interface {
	Shape
	Volume() float64
}

type Cube struct{ S float64 }

func (c Cube) Area() float64   { return 6 * c.S * c.S }
func (c Cube) Volume() float64 { return c.S * c.S * c.S }

func main() {
	var a, b Solid = Cube{1}, Cube{1}
	_ = a == b
}
`)
	if strings.Contains(rust, "Shape::__go_as_any(") {
		t.Fatalf("one embedded interface is not ambiguous and needs no qualified call:\n%s", rust)
	}
}

func TestInterfaceValueStoredElsewhereIsCopied(t *testing.T) {
	rust := transpileTypedRegression(t, `package main

type Shape interface{ Area() float64 }
type Rect struct{ W, H float64 }

func (r Rect) Area() float64 { return r.W * r.H }

type Holder struct{ S Shape }

func (h *Holder) Get() Shape { return h.S }

func main() {
	var x Shape = Rect{1, 2}
	y := x
	h := Holder{S: x}
	arr := []Shape{x}
	m := map[int]Shape{}
	m[1] = x
	e := arr[0]
	x = Rect{3, 3}
	_, _, _, _ = y, h, e, m
}
`)
	for _, want := range []string{
		"let mut y = { let __iface_value = x.borrow().clone(); Rc::new(RefCell::new(__iface_value)) };",
		"s: { let __iface_value = x.borrow().clone(); Rc::new(RefCell::new(__iface_value)) }",
		"vec![{ let __iface_value = x.borrow().clone(); Rc::new(RefCell::new(__iface_value)) }]",
		"let __map_value = { let __iface_value = x.borrow().clone(); Rc::new(RefCell::new(__iface_value)) };",
		"{ let __iface_value = self.s.borrow().clone(); Rc::new(RefCell::new(__iface_value)) }",
	} {
		if !strings.Contains(rust, want) {
			t.Errorf("an interface value stored somewhere new should be a copy, missing %q:\n%s", want, rust)
		}
	}
}

func TestInterfaceImplementorComparesByValueWithGoroutines(t *testing.T) {
	rust := transpileTypedConcurrentRegression(t, `package main

type Shape interface{ Area() float64 }
type Point struct{ X, Y float64 }
type Rect struct {
	Min  Point
	W, H float64
}

func (r Rect) Area() float64 { return r.W * r.H }

func main() {
	go func() {}()
	var a, b Shape = Rect{Point{1, 1}, 1, 2}, Rect{Point{1, 1}, 1, 2}
	_ = a == b
}
`)
	for _, want := range []string{"impl PartialEq for Rect", "impl PartialEq for Point"} {
		if !strings.Contains(rust, want) {
			t.Errorf("comparing Rect through an interface needs %q:\n%s", want, rust)
		}
	}
	if strings.Contains(rust, "panic!(\"interface comparison with uncomparable dynamic type\")") {
		t.Errorf("a comparable struct must not panic when compared through an interface:\n%s", rust)
	}
	// An interface value and its copy share the struct's field cells. The
	// comparison reads each side under its own lock; holding both at once
	// locks one mutex twice and hangs.
	if strings.Contains(rust, ".lock().unwrap(); let __right = other.") {
		t.Errorf("PartialEq must not hold both field locks at once:\n%s", rust)
	}
	if !strings.Contains(rust, "let __left = { self.w.lock().unwrap().as_ref().cloned() }; let __right = { other.w.lock().unwrap().as_ref().cloned() }; __left == __right") {
		t.Errorf("PartialEq should read each side under its own lock:\n%s", rust)
	}
}
