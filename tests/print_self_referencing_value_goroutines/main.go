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
}
