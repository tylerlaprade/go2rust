package main

import "fmt"

func main() {
	fmt.Println(apply(double, 21))
}

func double(x int) int {
	return x * 2
}
