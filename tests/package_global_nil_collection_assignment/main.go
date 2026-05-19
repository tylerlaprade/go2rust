package main

import "fmt"

var currentValues []int
var currentSeen map[string]int

func nilValues() []int {
	return nil
}

func nilSeen() map[string]int {
	return nil
}

func restore(values []int, seen map[string]int) {
	currentValues = values
	currentSeen = seen
}

func main() {
	restore(nilValues(), nilSeen())
	fmt.Println(len(currentValues), currentValues == nil, len(currentSeen), currentSeen == nil)

	restore([]int{1, 2}, map[string]int{"x": 3})
	fmt.Println(len(currentValues), currentValues == nil, len(currentSeen), currentSeen == nil)
}
