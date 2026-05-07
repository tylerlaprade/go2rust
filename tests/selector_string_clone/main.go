package main

import "fmt"

type Parts struct {
	left  string
	right string
}

func leftOf(parts Parts) string {
	return parts.left
}

func echo(value string) string {
	return value
}

func main() {
	parts := Parts{left: "go", right: "rust"}
	fmt.Println(leftOf(parts))
	fmt.Println(echo(parts.right))
}
