// len and cap as the values of map, slice, struct and array literals.
package main

import "fmt"

type stat struct {
	name string
	size int
}

func main() {
	words := []string{"go", "rust"}
	m := map[string]int{words[0]: len(words[0])}
	for _, w := range words {
		m[w] = len(w)
	}
	s := []int{len(words), cap(words)}
	st := stat{name: "x", size: len(words)}
	arr := [2]int{len(words), 1}
	fmt.Println(m, s, st, arr)
}
