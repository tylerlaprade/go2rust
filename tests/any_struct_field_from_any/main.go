package main

import "fmt"

type entry struct {
	value interface{}
}

func makeEntry(value interface{}) entry {
	return entry{value: value}
}

func main() {
	go func() {}()

	var value interface{} = "new"
	e := makeEntry(value)
	prev := e.value

	fmt.Println(e.value)
	fmt.Println(prev)
}
