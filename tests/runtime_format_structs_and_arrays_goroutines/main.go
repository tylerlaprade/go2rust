// In a program with goroutines: a format that only the run-time formatter
// takes (it has %q) prints %+v of a struct with field names, and %v of an
// anonymous struct and of an array.
package main

import (
	"fmt"
	"sync"
)

type State int

func (s State) String() string { return "st" }

type Point struct {
	X, Y int
	Tag  string
}

func main() {

	var s State
	fmt.Printf("%v %q\n", s, 'x')
	fmt.Printf("%+v %q\n", struct{ A, B int }{1, 2}, 'x')
	fmt.Printf("%v %q\n", struct{ A, B int }{3, 4}, 'y')
	fmt.Printf("%v %q\n", [2]bool{true}, 'x')
	fmt.Printf("%v|%+v %q\n", Point{1, 2, "p"}, Point{3, 4, "q"}, 'z')
	fmt.Printf("%T %q\n", s, 'x')
	var a any = Point{5, 6, "r"}
	fmt.Printf("%+v %q\n", a, "s")
	var wg sync.WaitGroup
	out := make([]string, 2)
	for i := range out {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()
			out[i] = fmt.Sprintf("%+v %q", Point{i, i, "g"}, 'g')
		}(i)
	}
	wg.Wait()
	fmt.Println(out)
}
