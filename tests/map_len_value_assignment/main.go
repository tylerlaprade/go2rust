package main

import "fmt"

func main() {
	values := []string{"a", "b"}
	counts := map[string]int{}
	counts["values"] = len(values)
	fmt.Println(counts["values"])
}
