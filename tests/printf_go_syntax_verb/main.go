package main

// %#v writes a value the way Go source would write it: the type name and
// field names of a struct, the elements of slices, arrays and maps (keys
// sorted), & before what a top-level pointer points at, GoString when the
// type has one, and the dynamic value of an interface.

import (
	"errors"
	"fmt"
	"os"
)

type Level int
type Name string
type Celsius float64
type IDs []int
type Tags map[string]int

type Point struct{ X, Y int }

type Shape interface{ Area() float64 }

type Square struct{ Side float64 }

func (s Square) Area() float64 { return s.Side * s.Side }

type Circle struct{ R float64 }

func (c *Circle) Area() float64 { return 3 * c.R * c.R }

// Version writes itself with GoString, which %#v prefers.
type Version struct{ Major, Minor int }

func (v Version) GoString() string { return fmt.Sprintf("v%d.%d", v.Major, v.Minor) }

type Handle struct{ id int }

func (h *Handle) GoString() string { return fmt.Sprintf("Handle#%d", h.id) }

type Base struct{ ID int }

type Record struct {
	Base
	*Point
	Name   string
	Tags   []string
	Scores map[string]int
	Flags  map[bool]string
	Grid   [2][2]int
	Next   *Record
	Data   []byte
	Count  uint
	Ratio  float64
	Any    any
	Err    error
	Shape  Shape
	OnDone func()
	Events chan int
	hidden int
}

type Pair[K comparable, V any] struct {
	Key K
	Val V
}

func size(t Tags) int { return len(t) }

func isNil(err error) bool { return err == nil }

func show(label string, v any) {
	fmt.Printf("%s: %#v\n", label, v)
}

func main() {
	fmt.Printf("%#v|%#v|%#v|%#v\n", Level(3), Name("bo"), Celsius(21.5), IDs{1, 2})
	fmt.Printf("%#v|%#v|%#v\n", Tags{"b": 2, "a": 1}, IDs(nil), Tags(nil))
	fmt.Printf("%#v|%#v|%#v|%#v\n", 42, -7, uint8(255), uintptr(16))
	fmt.Printf("%#v|%#v|%#v|%#v|%#v\n", 1.0, 2.5, 1e21, float32(0.1), "tab\there \"q\"")
	fmt.Printf("%#v|%#v|%#v\n", true, 'x', []rune("hi"))

	r := Record{
		Base:   Base{ID: 7},
		Name:   "first",
		Tags:   []string{"a", "b"},
		Scores: map[string]int{"z": 26, "a": 1, "m": 13},
		Flags:  map[bool]string{true: "yes", false: "no"},
		Grid:   [2][2]int{{1, 2}, {3, 4}},
		Data:   []byte("hi"),
		Count:  10,
		Ratio:  0.25,
		Any:    Point{1, 2},
		hidden: 9,
	}
	fmt.Printf("%#v\n", r)
	fmt.Printf("%#v\n", &Point{3, 4})
	var nilPoint *Point
	fmt.Printf("%#v %#v\n", nilPoint, []*Point{nil})

	fmt.Printf("%#v|%#v|%#v\n", Version{1, 2}, []Version{{2, 0}}, &Handle{5})

	var shapes []Shape
	shapes = append(shapes, Square{2}, &Circle{1})
	for _, s := range shapes {
		fmt.Printf("%#v\n", s)
	}
	var none Shape
	fmt.Printf("%#v %#v\n", none, r.Shape)

	show("int", 3)
	show("string", "s")
	show("struct", Square{3})
	show("pointer", &Square{4})
	show("slice", []int{1, 2})
	show("map", map[string]bool{"x": true})
	show("nil", nil)

	items := []any{1, "a", nil, 2.5, Point{5, 6}}
	fmt.Printf("%#v\n", items)

	keyed := map[Point]string{{2, 1}: "b", {1, 9}: "a", {1, 2}: "c"}
	fmt.Printf("%#v\n", keyed)

	type local struct {
		A int
		B []string
	}
	fmt.Printf("%#v|%#v\n", local{1, nil}, []local{{2, []string{"x"}}})

	p := Pair[string, int]{"k", 1}
	fmt.Printf("%#v %T\n", p, p)

	fmt.Printf("%#v|%#v|%#v\n", struct{}{}, struct{ A, B int }{1, 2}, [0]int{})

	err := fmt.Errorf("bad point %#v", Point{0, -1})
	fmt.Println(err)
	fmt.Fprintf(os.Stdout, "%d %#v %s\n", 5, []string{}, "end")
	fmt.Println(fmt.Sprintf("%#v", map[int]bool{3: true, 1: false}))
	fmt.Printf("%v %#v\n", errors.New("boom"), error(nil))

	// Typed nils as arguments, and a struct with a float field as a map key.
	var empty any = any(nil)
	fmt.Println(size(Tags(nil)), isNil(error(nil)), empty == nil)
	bySide := map[Square]int{{2}: 1, {1.5}: 2}
	fmt.Printf("%#v %v\n", bySide, bySide[Square{1.5}])
}
