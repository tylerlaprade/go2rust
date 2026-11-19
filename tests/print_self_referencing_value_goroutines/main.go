package main

import (
	"fmt"
	"strings"
)

// Node can point at itself through its interface field.
type Node struct {
	Name string
	Next interface{}
}

func render(v interface{}) string { return fmt.Sprintf("%v", v) }

type Point struct{ X int }

// Label prints itself by printing another value. That is a print call of its
// own, so the pointer it prints is expanded wherever the Label sits.
type Label struct{ X int }

func (l Label) String() string { return render(&Point{X: l.X}) }

// shape describes text around the addresses in it, which change from run to
// run: whether it starts with prefix, and how many addresses it holds.
func shape(text, prefix string) string {
	return fmt.Sprintf("%v %d", strings.HasPrefix(text, prefix), len(strings.Split(text, "0x"))-1)
}

func main() {
	// A goroutine makes every shared value a locked one. Printing a value
	// that points back to itself must not wait for a lock it already holds.
	done := make(chan string, 1)
	go func() { done <- "started" }()
	fmt.Println(<-done)

	loop := &Node{Name: "loop"}
	loop.Next = loop
	fmt.Println("pointer, boxed:", shape(render(loop), "&{loop 0x"))
	fmt.Println("pointer, typed:", shape(fmt.Sprintf("%v", loop), "&{loop 0x"))
	fmt.Println("struct value, boxed:", shape(render(*loop), "{loop 0x"))
	fmt.Println("struct value, typed:", shape(fmt.Sprintf("%v", *loop), "{loop 0x"))

	tail := &Node{Name: "tail"}
	text := render(Node{Name: "holder", Next: tail})
	fmt.Println("struct holding a pointer:", shape(text, "{holder 0x"), strings.Contains(text, "tail"))
	fmt.Println(Node{Name: "end"})

	fmt.Println(Node{Name: "row", Next: Label{7}})
	fmt.Println(render(&Node{Name: "row", Next: Label{7}}))

	// The elements of a slice are inside it. Spread into a print call they
	// are its arguments, and a pointer among them is expanded.
	var labels []interface{}
	labels = append(labels, Label{8}, &Point{2}, 1)
	fmt.Println("slice:", shape(fmt.Sprintf("%v", labels), "[&{8} 0x"))
	fmt.Println(labels...)
}
