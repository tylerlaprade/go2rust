package main

import "fmt"

func main() {
	var result []int
	set := func(values []int) {
		result = values
	}

	set([]int{4, 5})
	fmt.Println(result[0])
	fmt.Println(len(result))
}
