package main

import (
	"fmt"
	"sort"
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
	done := make(chan bool)
	go func() { done <- true }()
	<-done
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
	ids := []int{}
	for k := range seen {
		ids = append(ids, k.ID)
	}
	sort.Ints(ids)
	fmt.Println(len(seen), ids, seen[Pair{1, &s1}], seen[Pair{2, nil}], seen[Pair{1, nil}])
	u := map[struct{ p *int }]string{{&a}: "a", {&b}: "b"}
	fmt.Println(len(u), u[struct{ p *int }{&a}])
}
