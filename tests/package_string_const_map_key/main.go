package main

import (
	"fmt"

	"example.com/stringkey/dep"
)

func main() {
	seen := make(map[string]bool)
	seen[dep.Name] = true
	fmt.Println(seen[dep.Name])
}
