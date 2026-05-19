package main

import (
	"fmt"
	"go/build"
)

func main() {
	pkg, err := build.Default.Import("fmt", "", build.FindOnly)
	fmt.Println(err == nil, pkg.Goroot, pkg.ImportPath, pkg.Dir != "")
	fmt.Println(build.Default.GOROOT != "")
	fmt.Println(build.IsLocalImport("./pkg"), build.IsLocalImport("fmt"))
}
