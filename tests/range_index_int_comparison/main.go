package main

import "fmt"

func limit(values []int) int {
	return len(values)
}

func countUntilLimit(values []int) int {
	count := 0
	for i := range values {
		if i >= limit(values) {
			break
		}
		count++
	}
	return count
}

func main() {
	fmt.Println(countUntilLimit([]int{1, 2, 3}))
}
