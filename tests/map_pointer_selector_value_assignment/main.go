package main

import "fmt"

type Package struct {
	ID string
}

type loaderPackage struct {
	Package *Package
}

func main() {
	pkgs := map[string]*loaderPackage{
		"dep": {Package: &Package{ID: "dep"}},
	}
	imports := map[string]*Package{}

	imp := pkgs["dep"]
	imports["dep"] = imp.Package
	imp.Package.ID = "updated"

	fmt.Println(imports["dep"].ID)
	fmt.Println(imports["dep"] == imp.Package)
}
