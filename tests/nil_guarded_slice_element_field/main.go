package main

import "fmt"

type Node struct{ Value int }

func (n *Node) Get() int {
	if n == nil {
		return 0
	}
	return n.Value
}

type Holder struct{ Current *Node }

func main() {
	nodes := []Node{{Value: 9}}
	h := Holder{}
	h.Current = &nodes[0]
	fmt.Println(h.Current.Get())
}
