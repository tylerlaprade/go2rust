package main

import "fmt"

type Node struct {
	Value int
	Next  *Node
}

func (n *Node) Len() int {
	if n == nil {
		return 0
	}
	return 1 + n.Next.Len()
}

func main() {
	list := &Node{1, &Node{2, nil}}
	done := make(chan int)
	go func() { done <- list.Len() }()
	fmt.Println(<-done)
	var empty *Node
	fmt.Println(empty.Len())
}
