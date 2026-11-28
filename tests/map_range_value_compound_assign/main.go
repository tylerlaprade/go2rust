package main

import (
	"fmt"
	"sort"
)

type score int

type tally struct{ n int }

func main() {
	counts := map[string]int{"a": 2, "b": 5}
	n := 0
	for _, v := range counts {
		n += v
	}
	text := ""
	words := map[int]string{1: "x", 2: "y"}
	keys := []int{}
	for k := range words {
		keys = append(keys, k)
	}
	sort.Ints(keys)
	for _, k := range keys {
		text += words[k]
	}
	for _, w := range words {
		if w == "y" {
			text += w
		}
	}
	f := 1.0
	for _, v := range map[string]float64{"h": 0.5} {
		f *= v
		f -= v
	}
	var s score
	for _, v := range map[string]score{"s": 4} {
		s += v
	}
	t := tally{}
	for _, v := range counts {
		t.n += v
	}
	total := 0
	for _, v := range counts {
		func() { total += v }()
	}
	fmt.Println(n, text, f, s, t, total, counts)
}
