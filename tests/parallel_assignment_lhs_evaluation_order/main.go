package main

import "fmt"

var trace string

func target(label string, values []int) []int {
	trace += label + "|"
	return values
}

func index(label string, value int) int {
	trace += label + "|"
	return value
}

func result(label string, value int) int {
	trace += label + "|"
	return value
}

func main() {
	left := []int{0, 0}
	right := []int{0, 0}
	__go_lhs_sequence_0 := []int{10}
	__go_lhs_index_0 := 0
	target("left sequence", left)[index("left index", 0)], target("right sequence", right)[index("right index", 1)] = result("left result", __go_lhs_sequence_0[__go_lhs_index_0]), result("right result", 20)
	fmt.Println(trace)
	fmt.Println(left, right)
}
