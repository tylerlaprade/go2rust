package main

import "fmt"

type item struct {
	name string
}

func first(items []item) item {
	for _, item := range items {
		return item
	}
	return item{}
}

func main() {
	items := []item{{name: "alpha"}}
	fmt.Println(first(items).name)
}
