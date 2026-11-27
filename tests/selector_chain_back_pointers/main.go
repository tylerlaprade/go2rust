package main

import "fmt"

// A selector chain whose pointers lead back to a struct the chain already
// read, kid.parent.left with parent.left == kid, reads and writes as Go
// does: each pointer read out of a field ends its borrow before the pointee
// is borrowed.

type Node struct {
	val    int
	parent *Node
	left   *Node
}

type Dept struct {
	Name string
	Lead *Employee
}

type Employee struct {
	Name string
	Dept *Dept
}

type Team struct {
	Dept
}

func main() {
	root := &Node{val: 1}
	kid := &Node{val: 2, parent: root}
	root.left = kid
	fmt.Println(kid.parent.left.val)
	kid.parent.left.val = 5
	kid.parent.left.val++
	fmt.Println(kid.val, root.left.val)
	kid.parent.left.parent = nil
	fmt.Println(kid.parent == nil)
	root.left.parent = root
	root.left.parent.left = &Node{val: 9}
	fmt.Println(root.left.val, kid.parent.val)

	team := &Team{Dept: Dept{Name: "ops"}}
	team.Lead = &Employee{Name: "dan", Dept: &team.Dept}
	fmt.Println(team.Lead.Dept.Name, team.Lead.Dept.Lead.Name)
	team.Lead.Dept.Name = "ops3"
	fmt.Println(team.Name)
}
