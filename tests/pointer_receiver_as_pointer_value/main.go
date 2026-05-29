package main

type node struct {
	next *node
	v    int
}

func (x *node) walk() int {
	y := x
	sum := 0
	for y != nil {
		sum += y.v
		y = y.next
	}
	return sum
}

func main() {
	c := &node{v: 3}
	b := &node{v: 2, next: c}
	a := &node{v: 1, next: b}
	println(a.walk())
}
