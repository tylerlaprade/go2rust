// In a program with goroutines: a recovered nil pointer dereference is Go's runtime error, and prints as
// "runtime error: invalid memory address or nil pointer dereference".
package main

import (
	"fmt"
	"sync"
)

type Node struct {
	val  int
	next *Node
}

func safely(name string, f func()) {
	defer func() {
		if r := recover(); r != nil {
			fmt.Printf("%s: %v\n", name, r)
		}
	}()
	f()
}

func main() {
	var n *Node
	safely("field", func() { fmt.Println(n.val) })
	list := &Node{val: 1}
	safely("next", func() { fmt.Println(list.next.val) })
	m := map[string]*Node{}
	safely("missing", func() { m["x"].val = 3 })
	xs := []int{1, 2, 3}
	i := 5
	safely("index", func() { fmt.Println(xs[i]) })
	zero := 0
	safely("divide", func() { fmt.Println(10 / zero) })
	var wg sync.WaitGroup
	wg.Add(1)
	go func() {
		defer wg.Done()
		var w *Node
		safely("worker", func() { fmt.Println(w.next) })
	}()
	wg.Wait()
	fmt.Println("done")
}
