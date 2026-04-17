package main

import "fmt"

func main() {
	nums := []int{1, 2, 3}
	all := nums[:]
	fmt.Println(len(all), cap(all), all)

	s := "hello"
	fmt.Println(s[:])
}
