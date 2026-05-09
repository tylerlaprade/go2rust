package main

import "fmt"

func main() {
	values := make([]int, 3)
	for i := range values {
		values[i] = i
	}
	fmt.Println(values[0])
	fmt.Println(values[1])
	fmt.Println(values[2])
}
