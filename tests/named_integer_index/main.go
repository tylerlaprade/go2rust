package main

import "fmt"

type Kind int32

func pick(values []int, k Kind) int {
	return values[k]
}

func previous(values []int, k Kind) int {
	return values[k-1]
}

func (k Kind) methodPick(values []int) int {
	return values[k]
}

func main() {
	values := []int{10, 20, 30}
	var k Kind = 1
	fmt.Println(pick(values, k))
	fmt.Println(previous(values, k))
	fmt.Println(k.methodPick(values))
}
