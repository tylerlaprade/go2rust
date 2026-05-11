package main

import "fmt"

type Package struct {
	ID      string
	Imports map[string]*Package
}

func main() {
	done := make(chan bool, 1)
	root := &Package{
		Imports: map[string]*Package{
			"dep": {ID: "dep"},
		},
	}
	stubs := root.Imports
	root.Imports = map[string]*Package{}
	for path, pkg := range stubs {
		fmt.Println(path, pkg.ID)
	}
	done <- true
	fmt.Println(<-done)
}
