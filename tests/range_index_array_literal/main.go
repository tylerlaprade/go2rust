package main

import "fmt"

func main() {
	var pairs [][2]int
	for i, value := range []int{4, 5} {
		pairs = append(pairs, [2]int{i, value})
	}
	prev := [2]int{}
	for _, pair := range pairs {
		prev = pair
	}
	fmt.Println(pairs[0][0], pairs[1][1], prev[0], prev[1])
}
