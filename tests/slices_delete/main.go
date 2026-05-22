package main

import (
	"fmt"
	"slices"
)

type file struct {
	name string
}

func main() {
	files := []*file{{name: "a"}, {name: "b"}, {name: "c"}}
	files = slices.Delete(files, 1, 2)
	fmt.Println(len(files))
	fmt.Println(files[0].name)
	fmt.Println(files[1].name)
}
