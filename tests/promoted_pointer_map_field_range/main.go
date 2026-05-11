package main

import "fmt"

type Package struct {
	ID      string
	Imports map[string]*Package
}

type loaderPackage struct {
	*Package
	color int
}

func main() {
	done := make(chan bool, 1)
	base := &Package{
		ID: "root",
		Imports: map[string]*Package{
			"dep": {ID: "dep"},
		},
	}
	lpkg := &loaderPackage{
		Package: base,
	}
	stubs := lpkg.Imports
	lpkg.Imports = make(map[string]*Package, len(stubs))
	for importPath := range stubs {
		lpkg.Imports[importPath] = &Package{ID: "dep"}
	}
	fmt.Println("assigned")
	done <- true
	fmt.Println(<-done)
}
