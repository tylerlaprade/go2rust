package main

import "fmt"

type Spec interface {
	Name() string
}

type Concrete struct {
	N string
}

func (c Concrete) Name() string { return c.N }

func main() {
	specs := []Spec{Concrete{N: "alpha"}, Concrete{N: "omega"}}
	// short var decl from indexed wrapped interface slice element.
	lastSpec := specs[len(specs)-1]
	fmt.Println(lastSpec.Name())
}
