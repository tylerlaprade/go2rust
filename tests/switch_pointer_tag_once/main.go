package main

import "fmt"

func mark(label string, p *int) *int {
	fmt.Println(label)
	return p
}

func classify(p, q *int) {
	switch mark("tag", p) {
	case nil:
		fmt.Println("nil")
	case mark("first", q), mark("second", p):
		fmt.Println("match")
	default:
		fmt.Println("default")
	}
}

func main() {
	a, b := 1, 2
	classify(nil, &b)
	classify(&a, &a)
	classify(&a, &b)
}
