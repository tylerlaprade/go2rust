package main

import "fmt"

type node struct {
	v    int
	next *node
}

type list struct{ head *node }

// remove walks the links of its own receiver. The **T rewrite leaves this
// package as it is: a slot of a field of the receiver would rebind a copy.
func (l *list) remove(v int) bool {
	for pp := &l.head; *pp != nil; pp = &(*pp).next {
		if (*pp).v == v {
			*pp = (*pp).next
			return true
		}
	}
	return false
}

func (l *list) values() []int {
	var out []int
	for n := l.head; n != nil; n = n.next {
		out = append(out, n.v)
	}
	return out
}

func main() {
	l := &list{}
	for i := 5; i >= 1; i-- {
		l.head = &node{i, l.head}
	}
	fmt.Println(l.values())
	fmt.Println(l.remove(3), l.remove(5), l.remove(7))
	fmt.Println(l.values())
}
