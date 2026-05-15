package main

import "fmt"

var active []int

func swap(next []int) int {
	old := active
	active = next
	return len(old)*10 + len(active) + old[0] + active[0]
}

func main() {
	active = []int{1, 2}
	fmt.Println(swap([]int{3, 4, 5}))
}
