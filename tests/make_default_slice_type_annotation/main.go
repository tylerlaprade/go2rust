package main

import "fmt"

type item struct {
	name string
}

func main() {
	items := make([]item, 2)
	ptrs := make([]*item, len(items))
	fmt.Println(len(items), len(ptrs))
}
