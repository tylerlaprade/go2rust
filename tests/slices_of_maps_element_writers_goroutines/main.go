package main

import "fmt"

// A package-level map of []error writes each element as an error handle.
var table = map[string][]error{"row": {nil, fmt.Errorf("bad")}}

type rows []map[string]int

func main() {
	done := make(chan bool)
	go func() { done <- true }()
	<-done
	fmt.Println(len(table["row"]), table["row"][0] == nil, table["row"][1])

	// append to a named slice of maps keeps the map's handle.
	var s rows
	m := map[string]int{"a": 1}
	s = append(s, m)
	m["b"] = 2
	fmt.Println(len(s[0]), s[0])

	// An existing variable in a mixed short declaration takes the
	// element's map; a copy of its old map stays as it was.
	ms := []map[string]int{{"x": 1}}
	n0 := map[string]int{"old": 0}
	alias := n0
	n0, k := ms[0], 1
	n0["y"] = 2
	fmt.Println(ms, n0, alias, k)

	// A []any element asserted to a map shares the map, and a nil map
	// stays nil.
	var nm map[string]int
	values := []any{m, nm}
	held := []map[string]int{values[0].(map[string]int), values[1].(map[string]int)}
	m["c"] = 3
	fmt.Println(len(held[0]), held[1] == nil)

	// A range reads each element when its iteration starts, so a slot
	// replaced in an earlier iteration yields the new map.
	for i, row := range ms2() {
		fmt.Println(i, row)
	}

	// A ...map parameter holds the arguments' handles.
	fmt.Println(take(makeMap()), take(m, makeMap()), take(), m["seen"])
}

func makeMap() map[string]int { return map[string]int{"k": 1} }

func take(ms ...map[string]int) int {
	n := 0
	for _, m := range ms {
		m["seen"] = 1
		n += len(m)
	}
	return n
}

func ms2() []map[string]int {
	rs := []map[string]int{{"a": 1}, {"b": 2}}
	for i, r := range rs {
		if i == 0 {
			rs[1] = map[string]int{"x": 9}
		}
		r["seen"] = i
	}
	return rs
}
