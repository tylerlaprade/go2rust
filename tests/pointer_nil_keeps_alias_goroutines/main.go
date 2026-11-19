package main

import "fmt"

type Node struct {
	Value int
	Next  *Node
}

func (n *Node) Link() *Node {
	return n.Next
}

var global = &Node{Value: 5}

func (n *Node) ClearReceiver() (bool, int) {
	count := 0
	n, count = (nil), 5
	return n == nil, count
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

	observed := &Node{Value: 6}
	show := func() { fmt.Println(observed == nil) }
	observed = (nil)
	show()

	previous := global
	global = (nil)
	fmt.Println(global == nil, previous != nil)

	addressed := &Node{Value: 7}
	address := &(addressed)
	addressed = (nil)
	fmt.Println(*address == nil)

	parallelAddressed := &Node{Value: 8}
	parallelAddress := &parallelAddressed
	parallelAddressed, count = (nil), 4
	fmt.Println(*parallelAddress == nil, count)

	receiver := &Node{Value: 9}
	cleared, count := receiver.ClearReceiver()
	fmt.Println(cleared, count, receiver.Value)
}
