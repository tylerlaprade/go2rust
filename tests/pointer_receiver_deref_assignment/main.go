package main

import "fmt"

type Package struct {
	ID      string
	Name    string
	Imports map[string]*Package
}

func (p *Package) Reset(id, name string) {
	*p = Package{
		ID:   id,
		Name: name,
	}
	p.Imports = make(map[string]*Package)
	p.Imports["self"] = &Package{ID: p.ID}
}

func main() {
	pkg := &Package{
		ID:      "old",
		Name:    "Old",
		Imports: map[string]*Package{"dep": {ID: "dep"}},
	}
	pkg.Reset("new", "New")
	fmt.Println(pkg.ID)
	fmt.Println(pkg.Name)
	fmt.Println(pkg.Imports["self"].ID)
}
