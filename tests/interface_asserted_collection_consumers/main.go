package main

import "fmt"

func main() {
	var x interface{} = []int{1, 2, 3}
	fmt.Println(len(x.([]int)))
	boundSlice := x.([]int)
	fmt.Println(len(boundSlice))

	var y interface{} = map[string]int{"k": 3}
	fmt.Println(y.(map[string]int)["k"])
	boundMap := y.(map[string]int)
	fmt.Println(boundMap["k"])
}
