package main

import "fmt"

func addSuffix(s string) string {
	return s + "!"
}

func main() {
	value := "go"
	value = addSuffix(value)
	fmt.Println(value)
}
