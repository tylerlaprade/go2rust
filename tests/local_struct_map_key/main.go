package main

import (
	"fmt"
	"sort"
)

// A struct type declared inside a function works as a map key: it orders
// and compares by its fields, as one declared at package level does.

func main() {
	type key struct{ a, b int }
	grid := map[key]string{{1, 2}: "x"}
	grid[key{3, 4}] = "y"
	grid[key{1, 2}] += "!"
	fmt.Println(len(grid), grid[key{1, 2}], grid[key{0, 0}] == "")
	for _, k := range []key{{3, 4}, {1, 2}} {
		fmt.Println(k, grid[k])
	}

	type cell struct {
		row  int
		name string
	}
	counts := map[cell]int{}
	for _, c := range []cell{{1, "a"}, {0, "b"}, {1, "a"}} {
		counts[c]++
	}
	var keys []cell
	for c := range counts {
		keys = append(keys, c)
	}
	sort.Slice(keys, func(i, j int) bool { return keys[i].row < keys[j].row })
	fmt.Println(keys, counts[cell{1, "a"}])
	if _, ok := counts[cell{2, "z"}]; !ok {
		fmt.Println("no", cell{2, "z"})
	}
}
