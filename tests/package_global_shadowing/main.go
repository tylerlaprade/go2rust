package main

import "fmt"

var value int
var allValues = 5
var copiedValue int

func printValue(value int) {
	fmt.Println(value)
}

func main() {
	value = 3
	copiedValue = allValues
	printValue(7)
	fmt.Println(value)
	fmt.Println(copiedValue)
}
