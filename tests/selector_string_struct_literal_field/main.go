package main

import "fmt"

type Source struct {
	Name string
}

type Dest struct {
	Name string
}

func main() {
	src := &Source{Name: "original"}
	dst := Dest{Name: src.Name}
	src.Name = "changed"

	fmt.Println(dst.Name)
	fmt.Println(src.Name)
}
