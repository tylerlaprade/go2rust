// A float passed to panic prints as Go's print prints a float, and a
// float32 is widened to a float64 first. A recovered float keeps its value.
// A goroutine makes every variable a locked one and every panic value a
// boxed interface value.
package main

import "fmt"

func recovered(f func()) {
	defer func() { fmt.Println("recovered:", recover()) }()
	f()
}

func main() {
	started := make(chan string, 1)
	go func() { started <- "started" }()
	fmt.Println(<-started)
	recovered(func() { panic(2.5) })
	recovered(func() { panic(float32(0.25)) })
	panic(float32(1.5))
}
