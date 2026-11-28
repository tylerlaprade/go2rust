package main

import (
	"fmt"
	"sort"
)

// The elements of a slice or array of maps are the maps themselves: a copy
// of an element, an appended map and a copy of the slice all share them, a
// new element is a nil map, and writing to a nil one panics.

type Table struct {
	Rows []map[string]string
}

func (t *Table) Add(row map[string]string) { t.Rows = append(t.Rows, row) }

func main() {
	s := []map[string]int{{"x": 1}}
	s = append(s, map[string]int{"z": 3})
	s[0]["w"] = 0
	m := map[string]int{"k": 5}
	s = append(s, m)
	m["k"] = 6
	fmt.Println(s, len(s[1]))

	ms := make([]map[string]int, 3)
	fmt.Println(ms, ms[0] == nil, len(ms[1]))
	ms[0] = map[string]int{}
	ms[0]["a"] = 1
	alias := ms[0]
	alias["b"] = 2
	fmt.Println(ms[0], len(alias))
	for i := range ms {
		if ms[i] == nil {
			ms[i] = map[string]int{"i": i}
		}
	}
	for i, m := range ms {
		m["seen"] = i
	}
	fmt.Println(ms)

	cp := make([]map[string]int, 2)
	n := copy(cp, ms)
	cp[1]["copied"] = 1
	fmt.Println(n, ms[1]["copied"], cp)

	var arr [2]map[string]bool
	arr[0] = map[string]bool{"t": true}
	arr2 := arr
	arr2[0]["f"] = false
	fmt.Println(arr, arr2, arr[1] == nil)

	t := &Table{}
	row := map[string]string{"name": "x"}
	t.Add(row)
	t.Add(map[string]string{"name": "a"})
	row["extra"] = "y"
	sort.Slice(t.Rows, func(i, j int) bool { return t.Rows[i]["name"] < t.Rows[j]["name"] })
	fmt.Println(t.Rows, len(t.Rows[1]), *t)

	groups := map[string][]map[string]int{}
	groups["g"] = append(groups["g"], map[string]int{"v": 1}, nil)
	groups["g"][0]["w"] = 2
	fmt.Println(groups, len(groups["g"]), groups["g"][1] == nil)

	defer func() { fmt.Println("recovered:", recover()) }()
	nilMaps := []map[string]int{nil}
	nilMaps[0]["boom"] = 1
}
