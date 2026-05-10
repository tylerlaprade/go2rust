package main

import "fmt"

type item struct {
	name string
}

func main() {
	items := make(map[string]*item)
	items["alpha"] = &item{name: "first"}

	key := "beta"
	name := "second"
	items[key] = &item{name: name}

	fmt.Println(items["alpha"].name, items["beta"].name)
}
