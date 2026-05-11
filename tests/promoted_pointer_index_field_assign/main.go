package main

import "fmt"

type Package struct {
	Name    string
	Imports map[string]*Package
}

type loaderPackage struct {
	*Package
	color int
}

type loader struct {
	pkgs map[string]*loaderPackage
}

func main() {
	done := make(chan bool, 1)
	ld := &loader{
		pkgs: map[string]*loaderPackage{
			"root": {Package: &Package{Name: "root", Imports: map[string]*Package{"dep": {Name: "dep"}}}},
		},
	}
	for id := range ld.pkgs {
		ld.pkgs[id].Name = "cleared"
		ld.pkgs[id].Imports = nil
		fmt.Println(ld.pkgs[id].Package.Name)
		fmt.Println(ld.pkgs[id].Imports == nil)
	}
	done <- true
	fmt.Println(<-done)
}
