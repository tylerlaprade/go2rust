package main

import "fmt"

func collect(imports map[string]string) map[string]bool {
	seen := make(map[string]bool)
	for _, path := range imports {
		seen[path] = true
	}
	return seen
}

func main() {
	seen := collect(map[string]string{
		"fmt": "example.com/pkg",
	})
	fmt.Println(seen["example.com/pkg"])
}
