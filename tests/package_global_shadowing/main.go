package main

import "fmt"

var value int

func printValue(value int) {
	fmt.Println(value)
}

func main() {
	value = 3
	printValue(7)
	fmt.Println(value)
}
