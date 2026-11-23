package main

import "fmt"

func main() {
	var x interface{}
	x = map[string]int{"k": 3}
	m, ok := x.(map[string]int)
	fmt.Println(m["k"], ok)
}
