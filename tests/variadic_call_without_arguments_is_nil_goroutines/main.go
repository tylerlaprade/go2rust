package main

import (
	"fmt"
	"strings"
)

func f(xs ...int) bool { return xs == nil }

func g(prefix string, ms ...map[string]int) (int, bool) { return len(ms), ms == nil }

func sum(xs ...int) int {
	t := 0
	for _, x := range xs {
		t += x
	}
	return t
}

func grow(xs ...string) []string { return append(xs, "z") }

func pass(xs ...int) []int { return xs }

type T struct{}

func (T) m(xs ...any) int { return len(xs) }

func main() {
	done := make(chan bool)
	go func() { done <- true }()
	<-done
	fmt.Println(f(), f(1), sum(), sum(1, 2))
	fmt.Println(g("p"))
	fmt.Println(grow(), len(grow()), pass() == nil, pass(3))
	fmt.Println(T{}.m(), T{}.m(1, "a"))
	fmt.Println(strings.Join([]string{"a"}, ","), fmt.Sprint())
	h := f
	fmt.Println(h())
}
