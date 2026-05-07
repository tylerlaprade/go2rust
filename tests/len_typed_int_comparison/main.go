package main

import "fmt"

func consumedAll(values []int) bool {
	var i int
	for i = 0; i < len(values); i++ {
	}
	return i == len(values)
}

func lastIndex(values []int) int {
	return len(values) - 1
}

func main() {
	values := []int{1, 2, 3}
	fmt.Println(consumedAll(values))
	fmt.Println(lastIndex(values))
}
