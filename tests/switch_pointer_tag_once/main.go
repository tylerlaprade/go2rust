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

func shadow(p, _switch_val *int, _switch_val_ int) {
	switch p {
	case nil:
		fmt.Println("nil", _switch_val_)
	case _switch_val:
		fmt.Println("same", _switch_val_)
	default:
		fmt.Println("distinct", *_switch_val)
	}
}

func main() {
	a, b := 1, 2
	shadow(&a, &b, 7)
	shadow(&a, &a, 8)
	classify(nil, &b)
	classify(&a, &a)
	classify(&a, &b)
}
