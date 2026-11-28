package main

import (
	"fmt"

	"example.com/slicemapsparam/dep"
)

func main() {
	s := []map[string]int{{"a": 1}, {"b": 2}}
	m := dep.First(s)
	m["c"] = 3
	s[1]["d"] = 4
	dep.Pick(s, 1, "x")["e"] = 5
	fmt.Println(s, m)
	dep.Mark(s, "k")
	fmt.Println(s)
	dep.Swap(s)
	fmt.Println(s, m)
	dep.Fresh(s, 0)
	fmt.Println(s, dep.Count(s))
}
