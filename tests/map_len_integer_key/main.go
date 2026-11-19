package main

import "fmt"

func main() {
	m := map[int]int{0: 1}
	m[len(m)] += 5
	fmt.Println(m[0], m[1], len(m))
}
