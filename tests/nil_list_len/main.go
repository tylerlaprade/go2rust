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

func __go_nil_recv_node_len() int { return 42 }

func main() {
	fmt.Println(__go_nil_recv_node_len())
	list := &Node{1, &Node{2, nil}}
	fmt.Println(list.Len())
	var empty *Node
	fmt.Println(empty.Len())
}
