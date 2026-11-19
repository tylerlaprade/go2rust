package main

import "fmt"

type Node struct{ Value int }

func (n *Node) Get() int {
	if n == nil {
		return 0
	}
	return n.Value
}

func (n *Node) Add(delta int) int {
	if n == nil {
		return -delta
	}
	n.Value += delta
	return n.Value
}

func delta() int {
	fmt.Println("delta")
	return 3
}

type Holder struct{ Current *Node }

func main() {
	nodes := []Node{{Value: 9}}
	h := Holder{}
	fmt.Println(h.Current.Get())
	fmt.Println(h.Current.Add(delta()))
	h.Current = &nodes[0]
	fmt.Println(h.Current.Get())
	fmt.Println(h.Current.Add(delta()))
	fmt.Println(h.Current.Add(h.Current.Get()))
	fmt.Println(nodes[0].Value)
}
