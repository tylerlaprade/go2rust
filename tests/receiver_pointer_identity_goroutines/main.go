// In a program with goroutines: a pointer method that stores or returns its
// receiver gives the pointer it was called on.
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
	done := make(chan bool)
	go func() {
		k := &Node{name: "k"}
		c.Add(k)
		done <- k.parent == c && k.parent.parent == root
	}()
	fmt.Println(<-done)
}
