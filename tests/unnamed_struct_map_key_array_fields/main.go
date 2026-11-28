package main

import "fmt"

type Point struct{ X int }

func main() {
	m := map[struct{ P [1]Point }]bool{}
	m[struct{ P [1]Point }{[1]Point{{1}}}] = true
	fmt.Println(len(m), m[struct{ P [1]Point }{[1]Point{{1}}}])
	n := map[struct {
		Q [2][1]struct{ y int }
	}]int{}
	n[struct {
		Q [2][1]struct{ y int }
	}{}]++
	fmt.Println(len(n), n[struct {
		Q [2][1]struct{ y int }
	}{}])
}
