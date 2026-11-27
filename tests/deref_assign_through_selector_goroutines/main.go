package main

import "fmt"

// Assigning through a pointer read from a selector chain reads the pointer
// first, so it works when the pointer points back into the chain:
// *team.Lead.Dept with Lead.Dept == &team.Dept.

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
	size int
}

type Box struct{ items []*Dept }

func main() {
	// A goroutine makes every variable a locked one.
	started := make(chan string, 1)
	go func() { started <- "started" }()
	fmt.Println(<-started)

	team := &Team{Dept: Dept{Name: "ops"}}
	team.Lead = &Employee{Name: "dan", Dept: &team.Dept}
	*team.Lead.Dept = Dept{Name: "ops2"}
	fmt.Println(team.Name, team.Lead == nil)

	d := &Dept{Name: "a"}
	d.Lead = &Employee{Name: "e", Dept: d}
	*d.Lead.Dept = Dept{Name: "b", Lead: d.Lead}
	fmt.Println(d.Name, d.Lead.Name)

	b := &Box{items: []*Dept{d}}
	*b.items[0] = Dept{Name: "c"}
	fmt.Println(d.Name, d.Lead == nil)
}
