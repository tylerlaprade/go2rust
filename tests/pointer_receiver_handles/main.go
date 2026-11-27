// A pointer method that hands out its receiver, or walks it along a list,
// works on the struct its caller holds: a map or pointer field it sets
// through the handed-out pointer reaches the caller, and the pointer is the
// caller's, as a map key or in a registry.
package main

import (
	"fmt"
	"sort"
)

type Node struct {
	name string
	next *Node
	tags map[string]int
}

// Last walks the list from n and tags the node it ends on.
func (n *Node) Last() *Node {
	for n.next != nil {
		n = n.next
	}
	n.tags = map[string]int{"last": 1}
	return n
}

// Link appends a node after n and returns n, for chaining.
func (n *Node) Link(name string) *Node {
	n.next = &Node{name: name}
	return n
}

type Builder struct {
	parts map[string]int
	next  *Builder
}

func (b *Builder) Add(k string) *Builder {
	if b.parts == nil {
		b.parts = map[string]int{}
	}
	b.parts[k]++
	return b
}

func (b *Builder) Chain() *Builder {
	b.next = &Builder{}
	return b.next
}

type Dept struct {
	Name  string
	Staff []*Employee
	Lead  *Employee
}

type Employee struct {
	Name string
	Dept *Dept
}

func (d *Dept) Hire(e *Employee) {
	e.Dept = d
	d.Staff = append(d.Staff, e)
}

func (d *Dept) Promote() *Dept {
	d.Lead = d.Staff[0]
	return d
}

type Registrar interface{ Register() }

var registry []*Dept

func (d *Dept) Register() { registry = append(registry, d) }

type Team struct {
	Dept
	size int
}

func main() {
	list := &Node{name: "a", next: &Node{name: "b", next: &Node{name: "c"}}}
	last := list.Last()
	fmt.Println(last.name, list.next.next.tags["last"], last == list.next.next, list.next.name)

	var head Node
	head.Link("x").Link("y")
	fmt.Println(head.next.name)

	var bld Builder
	bld.Add("x").Add("y").Add("x")
	keys := []string{}
	for k := range bld.parts {
		keys = append(keys, k)
	}
	sort.Strings(keys)
	fmt.Println(keys, bld.parts["x"])
	bld.Chain().Add("deep")
	fmt.Println(bld.next != nil && bld.next.parts["deep"] == 1)

	eng := &Dept{Name: "eng"}
	for _, e := range []*Employee{{Name: "ann"}, {Name: "bob"}, {Name: "cat"}} {
		eng.Hire(e)
	}
	byDept := map[*Dept][]string{}
	for _, e := range eng.Staff {
		byDept[e.Dept] = append(byDept[e.Dept], e.Name)
	}
	fmt.Println(byDept[eng], len(byDept))
	fmt.Println(eng.Promote().Lead.Name, eng.Lead != nil)

	var r Registrar = eng
	r.Register()
	register := eng.Register
	register()
	fmt.Println(len(registry), registry[0] == eng, registry[1] == eng)
	// A map keyed by the pointer tells the registered pointers from
	// copies of the struct.
	registered := map[*Dept]int{}
	for _, d := range registry {
		registered[d]++
	}
	fmt.Println(len(registered), registered[eng])

	team := &Team{Dept: Dept{Name: "ops"}}
	team.Hire(&Employee{Name: "dan"})
	fmt.Println(team.Promote().Lead.Name, team.Lead.Dept == &team.Dept)
}
