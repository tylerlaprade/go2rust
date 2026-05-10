package main

import (
	"fmt"

	"example.com/packagevariadic/label"
)

func main() {
	fmt.Println(label.Count("empty"))
	fmt.Println(label.Count("full", "alpha", "beta"))
}
