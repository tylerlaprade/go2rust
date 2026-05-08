package main

import "fmt"

type bag struct {
	values []int
}

func (b bag) Sum() int {
	sum := 0
	for _, value := range b.values {
		sum += value
	}
	return sum
}

func main() {
	bag := bag{values: []int{1, 2, 3}}
	fmt.Println(bag.Sum())
}
