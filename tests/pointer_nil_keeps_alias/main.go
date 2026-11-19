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
}
