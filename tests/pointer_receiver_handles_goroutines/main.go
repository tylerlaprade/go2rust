// A pointer method that hands out its receiver, or walks it along a list,
// works on the struct its caller holds: a map or pointer field it sets
// through the handed-out pointer reaches the caller, and the pointer is the
// caller's, as a map key or in a registry.
// A goroutine makes every variable a locked one, and the methods started
// with go and defer run on the caller's struct too.
package main

import (
	"fmt"
	"sort"
	"sync"
)

type Srv struct {
	seen map[string]int
	wg   sync.WaitGroup
}

func (s *Srv) loop()  { defer s.wg.Done(); s.seen = map[string]int{"x": 1} }
func (s *Srv) reset() { s.seen = nil }
func (s *Srv) Start() { s.wg.Add(1); go s.loop(); s.wg.Wait() }
func (s *Srv) Stop()  { defer s.reset() }

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

var lastVisited *Node

// Bind returns Visit bound to n; the method value takes n's handle.
func (n *Node) Bind() func(func()) { return n.Visit }

// Visit sets a field and then runs f, which reads it through the caller's
// pointer.
func (n *Node) Visit(f func()) {
	n.tags = map[string]int{"seen": 1}
	f()
	lastVisited = n
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
	srv := &Srv{}
	srv.Start()
	fmt.Println(len(srv.seen), srv.seen["x"])
	srv.Stop()
	fmt.Println(srv.seen == nil)

	list := &Node{name: "a", next: &Node{name: "b", next: &Node{name: "c"}}}
	last := list.Last()
	fmt.Println(last.name, list.next.next.tags["last"], last == list.next.next, list.next.name)

	visit := (*Node).Visit
	visit(list, func() { fmt.Println(list.tags["seen"]) })
	fmt.Println(lastVisited == list)
	nodes := []*Node{{name: "p"}, {name: "q"}}
	for _, nd := range nodes {
		nd.Visit(func() { fmt.Println(nd.name, nd.tags["seen"]) })
	}
	for i := range nodes {
		nodes[i].Visit(func() { fmt.Println(nodes[i].tags["seen"]) })
	}
	fmt.Println(lastVisited == nodes[1])

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

	// Element receivers of value slices, arrays and pointer maps; the map key
	// names a runtime type to show that the choice reads no generated text.
	values := []Node{{name: "v"}}
	values[0].Visit(func() { fmt.Println(values[0].tags["seen"]) })
	var fixed [1]Node
	fixed[0].Visit(func() { fmt.Println(fixed[0].tags["seen"]) })
	byKey := map[string]*Node{"GoPtr": {name: "k"}}
	byKey["GoPtr"].Visit(func() { fmt.Println(byKey["GoPtr"].tags["seen"]) })
	fmt.Println(lastVisited == byKey["GoPtr"])

	// The pointer a method keeps is the caller's own: replacing the whole
	// pointee through it shows through the caller's pointer, which a
	// handle to a copy sharing the field cells would not.
	visit(list, func() {})
	*lastVisited = Node{name: "z"}
	fmt.Println(list.name)
	fresh := []*Node{{name: "r"}, {name: "s"}}
	for i := range fresh {
		fresh[i].Visit(func() { fmt.Println(fresh[i].name, fresh[i].tags["seen"]) })
	}
	*lastVisited = Node{name: "swapped"}
	fmt.Println(fresh[0].name, fresh[1].name)
	leadDept := team.Lead.Dept
	*leadDept = Dept{Name: "ops2"}
	fmt.Println(team.Name)

	// A method value bound to a pointer holds no borrow of it while the
	// method runs, so a callback can read the pointer: through a method
	// value a method returns for its own receiver, and directly.
	bound := fresh[0].Bind()
	bound(func() { fmt.Println("bound", fresh[0].tags["seen"]) })
	plain := fresh[1].Visit
	plain(func() { fmt.Println("plain", fresh[1].tags["seen"]) })

	// A method that passes its receiver on hands over the caller's
	// pointer, so pointer, map and channel fields set through it stick
	// (DEV-216).
	holder := &List{}
	holder.reset(5)
	fmt.Println(holder.head != nil, *holder.head)
	holder.init()
	holder.ch <- 3
	fmt.Println(len(holder.m), <-holder.ch)
}

type List struct {
	head *int
	m    map[string]int
	ch   chan int
}

func setHead(l *List, p *int) { l.head = p }

func (l *List) reset(v int) { setHead(l, &v) }

func fill(l *List) {
	l.m = map[string]int{"a": 1}
	l.ch = make(chan int, 1)
}

func (l *List) init() {
	fill(l)
	func(x *List) { x.m["b"] = 2 }(l)
}
