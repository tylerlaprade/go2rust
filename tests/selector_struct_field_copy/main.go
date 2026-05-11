package main

import "fmt"

type Module struct {
	Path string
}

type Source struct {
	Module Module
}

type Dest struct {
	Module Module
}

func main() {
	src := &Source{Module: Module{Path: "old"}}
	dst := Dest{Module: src.Module}
	src.Module.Path = "new"
	fmt.Println(dst.Module.Path)
	fmt.Println(src.Module.Path)
}
