package main

import (
	"fmt"
	"sort"
)

type Counts map[string]int

func main() {
	m := make(Counts)
	m["a"] = 1
	m["b"] = 2
	m["c"] = 3

	keys := make([]string, 0, len(m))
	for k := range m {
		keys = append(keys, k)
	}
	sort.Strings(keys)
	for _, k := range keys {
		fmt.Println(k, m[k])
	}
}
