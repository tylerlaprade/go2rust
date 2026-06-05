package main

import "fmt"

type item struct {
	name string
}

func pop(items []*item) any {
	x := items[0]
	return x
}

func main() {
	items := []*item{{name: "alpha"}}
	p := pop(items).(*item)
	p.name = "beta"
	fmt.Println(items[0].name, p.name)
}
