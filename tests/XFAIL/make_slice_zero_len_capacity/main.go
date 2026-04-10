package main

import "fmt"

func main() {
	s := make([]int, 0, 4)
	fmt.Println(len(s), cap(s), s == nil)
	s = append(s, 7, 8)
	fmt.Println(len(s), cap(s), s)
}
