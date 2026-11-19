package main

import "fmt"

type Numbers []int

func main() {
	m := map[string]Numbers{"a": Numbers{1}, "b": Numbers{2}}
	a, b := m["a"], m["b"]
	fmt.Println(len(a), a[0], len(b), b[0])
	a[0] = 7
	a = nil
	saved := m["a"]
	fmt.Println(saved[0], b[0])
}
