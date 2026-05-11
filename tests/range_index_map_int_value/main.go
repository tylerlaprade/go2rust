package main

import "fmt"

func roots() []string {
	return []string{"alpha", "beta"}
}

func main() {
	rootMap := map[string]int{}
	for i, root := range roots() {
		rootMap[root] = i
	}
	fmt.Println(rootMap["alpha"])
	fmt.Println(rootMap["beta"])
}
