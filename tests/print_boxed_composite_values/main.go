package main

import (
	"fmt"
	"strings"
)

type Point struct{ X, Y int }

type Status struct{ Code int }

func (s Status) String() string { return fmt.Sprintf("status %d", s.Code) }

type NotFound struct{ Name string }

func (e NotFound) Error() string { return e.Name + " not found" }

// Node can point at itself through its interface field.
type Node struct {
	Name string
	Next interface{}
}

func render(v interface{}) string { return fmt.Sprintf("%v", v) }

// show prints whatever it is given. The value arrives as an interface, so
// the printer has to find out what it holds.
func show(v interface{}) {
	fmt.Println(v)
}

func main() {
	show(42)
	show("hi")
	show(2.5)
	show(true)
	show([]int{1, 2, 3})
	show([]string{"a", "b"})
	show([]bool{true, false})
	show([]float64{1.5, 2})
	show([]Point{{1, 2}, {3, 4}})
	show(map[string]int{"b": 2, "a": 1})
	show(map[int]string{2: "two", 1: "one"})
	show(map[string]Point{"a": {1, 2}})
	show(Point{1, 2})
	show(&Point{3, 4})
	show(Status{404})
	show(NotFound{"x"})

	var none []int
	show(none)
	var nowhere *Point
	show(nowhere)

	xs := []int{4, 5}
	p := &Point{6, 7}
	var boxed interface{} = xs
	fmt.Println("println:", boxed, Point{5, 6})
	fmt.Printf("printf: %v %d\n", boxed, 7)
	boxed = p
	fmt.Printf("printf: %v\n", boxed)

	items := []interface{}{[]int{1, 2}, Point{7, 8}, map[string]int{"k": 1}, "s", 3}
	for _, item := range items {
		fmt.Println("item:", item)
	}

	names := []string{"x"}
	byName := map[string]interface{}{"point": Point{9, 9}, "list": names}
	fmt.Println(byName["point"], byName["list"])

	// A pointer below the top level prints as its address, so a value that
	// points back to itself prints once and stops. The address changes from
	// run to run; check the shape around it.
	loop := &Node{Name: "loop"}
	loop.Next = loop
	text := render(loop)
	fmt.Println("cycle:", strings.HasPrefix(text, "&{loop 0x"), strings.HasSuffix(text, "}"), len(strings.Split(text, "loop"))-1)
	tail := &Node{Name: "tail"}
	text = render(&Node{Name: "head", Next: tail})
	fmt.Println("chain:", strings.HasPrefix(text, "&{head 0x"), strings.Contains(text, "tail"))
	var noNode *Node
	show(&Node{Name: "end", Next: noNode})
	show(p)
}
