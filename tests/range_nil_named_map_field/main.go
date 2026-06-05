package main

import "fmt"

type namedMap map[string]int

type holder struct {
	values namedMap
}

func main() {
	var h holder
	count := 0
	for key := range h.values {
		fmt.Println("unexpected", key)
		count++
	}
	fmt.Println("count", count)
}
