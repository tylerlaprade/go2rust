package main

import (
	"fmt"
)

type Package struct {
	ID string
}

func (p *Package) String() string {
	return p.ID
}

func main() {
	done := make(chan bool, 1)
	stack := []*Package{{ID: "root"}, {ID: "dep"}}
	fmt.Println(fmt.Sprintf("cycle: %s", stack))
	done <- true
	fmt.Println(<-done)
}
