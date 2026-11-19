package main

import "fmt"

type Node struct {
	Value int
	Next  *Node
}

func (n *Node) Link() *Node {
	return n.Next
}

func main() {
	done := make(chan bool)
	go func() { done <- true }()
	<-done

	a := &Node{Value: 1}
	b := a
	b = nil
	fmt.Println(a != nil, b == nil)

	shared := &Node{Value: 2}
	first := &Node{Next: shared}
	second := &Node{Next: shared}
	first.Next = nil
	fmt.Println(first.Next == nil, second.Next != nil)

	c := &Node{Next: &Node{Value: 3}}
	p := c.Link()
	p = nil
	fmt.Println(c.Next != nil, p == nil)

	parallel := a
	count := 0
	parallel, count = (nil), 2
	fmt.Println(a != nil, parallel == nil, count)

	second.Next, count = nil, 3
	fmt.Println(shared != nil, second.Next == nil, count)

	captured := &Node{Value: 4}
	clear := func() { captured = nil }
	clear()
	fmt.Println(captured == nil)
}
