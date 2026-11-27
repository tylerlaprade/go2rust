package main

import (
	"fmt"
	"sort"
)

// A pointer to a map, from &m or from a map literal, shares the map with
// what it points at, through *p, (*p)[k], range, delete and assignment, for
// unnamed maps and defined map types alike.

type Lists map[string][]int

type Index struct {
	words *map[string]int
}

func size(p *map[string]int) int { return len(*p) }

func add(p *map[string]int, k string) { (*p)[k]++ }

func grow(p *Lists) { (*p)["g"] = []int{1} }

func keys(p *map[string]int) []string {
	var ks []string
	for k := range *p {
		ks = append(ks, k)
	}
	sort.Strings(ks)
	return ks
}

func main() {
	p := &map[string]int{"a": 1}
	fmt.Println(size(p))
	add(p, "b")
	add(p, "a")
	fmt.Println(len(*p), (*p)["a"], (*p)["b"], keys(p))

	raw := map[string]int{"x": 1}
	q := &raw
	add(q, "y")
	delete(*q, "x")
	fmt.Println(size(q), len(raw), keys(&raw))

	*p = map[string]int{"z": 9}
	fmt.Println(*p, len(*p))
	*p = nil
	fmt.Println(*p == nil, len(*p))

	idx := Index{words: &map[string]int{}}
	count := func(w string) { (*idx.words)[w]++ }
	for _, w := range []string{"go", "rust", "go"} {
		count(w)
	}
	fmt.Println(*idx.words, keys(idx.words))

	l := Lists{"a": {1}}
	lp := &l
	grow(lp)
	fmt.Println(len(*lp), len(l), (*lp)["g"])
	*lp = Lists{"z": {9}}
	fmt.Println(l, *lp)
}
