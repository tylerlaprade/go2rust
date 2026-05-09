package main

import "fmt"

type Node struct {
	name string
}

type Link struct {
	owner *Node
}

func (n *Node) Link() Link {
	return Link{owner: n}
}

func (l Link) OwnerName() string {
	return l.owner.name
}

func main() {
	node := Node{name: "root"}
	fmt.Println(node.Link().OwnerName())
}
