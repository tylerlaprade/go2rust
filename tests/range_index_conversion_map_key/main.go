package main

import "fmt"

func values() []string {
	return []string{"a", "b"}
}

func main() {
	m := map[string]uint64{}
	for i, v := range values() {
		m[v] = uint64(i)
	}
	fmt.Println(m["a"], m["b"])
}
