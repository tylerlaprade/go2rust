package main

import (
	"fmt"
	"sync"
)

type K struct{ P *int }

type Node struct {
	Name string
	Next *Node
}

type Pair struct {
	ID  int
	Ref *string
}

func main() {
	a, b := 1, 1
	m := map[K]int{}
	m[K{&a}]++
	m[K{&b}]++
	m[K{&a}]++
	fmt.Println(len(m), K{&a} == K{&b}, K{&a} == K{&a}, K{} == K{}, K{&a} == K{})
	fmt.Println(m[K{&a}], m[K{&b}], m[K{}])
	n1 := &Node{Name: "x"}
	n2 := &Node{Name: "x"}
	fmt.Println(Node{"a", n1} == Node{"a", n1}, Node{"a", n1} == Node{"a", n2})
	s1, s2 := "s", "s"
	seen := map[Pair]bool{{1, &s1}: true, {1, &s2}: true, {2, nil}: true}
	byID := map[int]int{}
	for k := range seen {
		byID[k.ID]++
	}
	fmt.Println(len(seen), byID, seen[Pair{1, &s1}], seen[Pair{2, nil}], seen[Pair{1, nil}])
	u := map[struct{ p *int }]string{{&a}: "a", {&b}: "b"}
	fmt.Println(len(u), u[struct{ p *int }{&a}])

	// A *sync.Mutex field is compared, and ordered, by the mutex it points to.
	mu1, mu2 := &sync.Mutex{}, &sync.Mutex{}
	locks := map[Guarded]int{}
	locks[Guarded{"x", mu1}]++
	locks[Guarded{"x", mu2}]++
	locks[Guarded{"x", mu1}]++
	fmt.Println(len(locks), locks[Guarded{"x", mu1}], Guarded{"x", mu1} == Guarded{"x", mu2})
}

type Guarded struct {
	Name string
	Mu   *sync.Mutex
}
