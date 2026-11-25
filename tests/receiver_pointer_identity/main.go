// A pointer method that stores or returns its receiver gives the pointer it
// was called on: c.parent == root after root.Add(c).
package main

import "fmt"

type Node struct {
	name   string
	parent *Node
	kids   []*Node
}

func (p *Node) Add(c *Node) { c.parent = p; p.kids = append(p.kids, c) }

func (p *Node) Self() *Node { return p }

func main() {
	root := &Node{name: "root"}
	c := &Node{name: "c"}
	root.Add(c)
	fmt.Println(c.parent == root, c.parent.name, len(root.kids), root.Self() == root)
	c.parent.name = "renamed"
	fmt.Println(root.name)
}
