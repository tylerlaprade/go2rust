package main

import "fmt"

type Package struct {
	ID string
}

type LoaderPackage struct {
	Imports map[string]*Package
}

func main() {
	ids := map[string]bool{"C": true, "pkg": true}
	pkg := &LoaderPackage{Imports: make(map[string]*Package)}

	for id := range ids {
		if id == "C" {
			continue
		}
		pkg.Imports[id] = &Package{ID: id}
	}

	fmt.Println(len(pkg.Imports))
	fmt.Println(pkg.Imports["pkg"].ID)
}
