package main

import "fmt"

func main() {
	var arr [3]int
	arr[0] = 10
	arr[1] = 20
	arr[2] = 30

	fmt.Println("Array elements:")
	for i := 0; i < len(arr); i++ {
		fmt.Println(arr[i])
	}

	// Array initialization
	nums := [4]int{1, 2, 3, 4}
	fmt.Println("Initialized array:")
	for _, num := range nums {
		fmt.Println(num)
	}
}
