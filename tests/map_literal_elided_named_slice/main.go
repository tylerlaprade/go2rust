package main

import "fmt"

type Numbers []int

func main() {
	m := map[string]Numbers{"a": {1, 3: 4}, "empty": {}, "explicit": Numbers{2}}
	a := m["a"]
	empty := m["empty"]
	explicit := m["explicit"]
	fmt.Println(len(a), a[0], a[1], a[3], len(empty), empty == nil, explicit[0])
}
