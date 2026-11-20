// %+v prints a struct with its field names, and a struct inside it, behind
// a pointer or in a slice the same way. It printed Rust's Debug form.
package main

import (
	"errors"
	"fmt"
)

type Point struct{ X, Y int }

type Base struct{ ID int }

type Line struct {
	Base
	From, To Point
	Label    string
	Tags     []string
	Weight   float64
	Ok       bool
}

type Named struct{ N int }

func (n Named) String() string { return fmt.Sprintf("N%d", n.N) }

type Config struct {
	Name    string
	Retries int
	Verbose bool
}

func main() {
	p := Point{1, 2}
	l := Line{Base{7}, Point{0, 0}, Point{3, 4}, "diag", []string{"a", "b"}, 2.5, true}
	fmt.Printf("%+v\n", p)
	fmt.Printf("%+v\n", l)
	fmt.Printf("%v\n", l)
	s := fmt.Sprintf("p=%+v n=%+v i=%+v d=%+d str=%+v", p, Named{3}, 5, 6, "x")
	fmt.Println(s)
	fmt.Printf("%+v\n", errors.New("boom"))
	pts := []Point{{1, 2}, {3, 4}}
	fmt.Printf("%+v %v\n", pts, pts)
	pp := &p
	fmt.Printf("%+v %v\n", pp, pp)
	fmt.Printf("%+v\n", struct{ A, B int }{1, 2})
	cfg := Config{Name: "svc", Retries: 3}
	fmt.Printf("config: %+v\n", cfg)
	fmt.Println(fmt.Sprintf("%v|%+v", cfg, cfg))
}
