package main

import "fmt"

func main() {
	s := []int{1, 2}
	slices := make(chan []int, 1)
	slices <- s
	receivedSlice := <-slices
	receivedSlice[0] = 9
	receivedSlice = append(receivedSlice, 3)
	fmt.Println(s, receivedSlice)

	m := map[string]int{"a": 1}
	maps := make(chan map[string]int, 1)
	maps <- m
	receivedMap := <-maps
	receivedMap["a"] = 7
	fmt.Println(m["a"], receivedMap["a"])
}
