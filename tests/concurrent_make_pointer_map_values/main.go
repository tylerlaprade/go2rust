package main

import "fmt"

type node struct {
	value int
}

func main() {
	go func() {}()
	n := &node{value: 5}
	byName := make(map[string]*node)
	byName["x"] = n
	fmt.Println(byName["x"].value)
}
