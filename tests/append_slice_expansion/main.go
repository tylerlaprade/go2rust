package main

import "fmt"

func main() {
	dst := []int{1, 2}
	src := []int{3, 4, 5}
	dst = append(dst, src...)
	fmt.Println(dst)

	words := []string{"go", "to", "rust"}
	prefix := []string{"transpile"}
	prefix = append(prefix, words...)
	fmt.Println(prefix)
}
