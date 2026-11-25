package main

import "fmt"

type term struct{ v int }

func (x *term) union(y *term) (*term, *term) {
	if x == nil {
		return y, nil
	}
	return &term{x.v + y.v}, nil
}

func main() {
	xl := []*term{{1}, {2}, {3}}
	for i, xi := range xl {
		for j := i + 1; j < len(xl); j++ {
			xj := xl[j]
			if u1, u2 := xi.union(xj); u2 == nil {
				xi = u1
			}
		}
		fmt.Println(i, xi.v)
	}
}
