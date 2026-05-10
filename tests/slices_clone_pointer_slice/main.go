package main

import (
	"fmt"
	"slices"
)

type pkg struct {
	path string
}

func main() {
	pkgs := []*pkg{{path: "root"}, {path: "dep"}}
	list := slices.Clone(pkgs[1:])
	fmt.Println(len(list))
}
