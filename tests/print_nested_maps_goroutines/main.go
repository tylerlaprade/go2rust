// A map whose values are maps prints as fmt does, in key order, also when
// the inner maps hold slices or structs, and a nil inner slice prints as [].
// A slice of maps prints each map the same way, and a nil map in a slice
// literal is an empty map.
// A goroutine makes every variable a locked one.
package main

import "fmt"

type Point struct{ X, Y int }

func main() {
	started := make(chan string, 1)
	go func() { started <- "started" }()
	fmt.Println(<-started)
	groups := map[string]map[string][]int{}
	for i, k := range []string{"a", "b", "a"} {
		if groups[k] == nil {
			groups[k] = map[string][]int{}
		}
		groups[k]["n"] = append(groups[k]["n"], i)
	}
	groups["c"] = map[string][]int{"empty": nil}
	fmt.Println(groups)

	byID := map[int]map[string][]string{2: {"tags": {"x", "y"}}, 1: {}}
	flags := map[string]map[string]bool{"t": {"ok": true, "bad": false}}
	fmt.Println(byID, flags)

	grid := map[string]map[string]Point{"row": {"b": {3, 4}, "a": {1, 2}}}
	fmt.Println(grid)
	fmt.Printf("%v|%v\n", grid["row"], groups["a"])

	rows := []map[string]int{{"x": 1}, nil, {}}
	rows[1] = map[string]int{"y": 2}
	lists := []map[string][]int{{"a": {1}, "b": nil}}
	fmt.Println(rows, len(rows[2]), lists)
}
