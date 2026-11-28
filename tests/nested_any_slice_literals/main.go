package main

import "fmt"

type pair struct {
	Name string
	N    int
}

func main() {
	// The inner literals box their elements, whether their type is elided
	// or written out.
	rows := [][]any{{1, "a"}, {2}}
	typed := [][]any{[]any{1, "a"}}
	fmt.Println(len(rows), len(rows[0]), len(typed))
	fmt.Println(rows, typed)
	for _, row := range rows {
		fmt.Println(row...)
		fmt.Println(row)
	}
	first := rows[0]
	fmt.Println(first, first[1].(string))
	if v, ok := rows[0][1].(string); ok {
		fmt.Println("ok", v)
	}
	if _, ok := rows[0][0].(string); !ok {
		fmt.Println("not a string")
	}
	fmt.Print(fmt.Sprintln(rows[1]...))

	grid := [2][]any{{1.5, true}, {nil}}
	fmt.Println(grid, len(grid[0]), grid[1][0] == nil, grid[0][1].(bool))

	byKey := map[string][]any{"k": {1, "x"}, "e": {}}
	fmt.Println(byKey, len(byKey["k"]), byKey["k"][1])

	deep := [][][]any{{{1}, {"b", 2}}}
	fmt.Println(deep, deep[0][1][0], len(deep[0]))

	// Values go in as Go copies them; a pointer shares its target.
	n := 7
	p := pair{"p", 3}
	mixed := [][]any{{n, p, &n}, {[]int{1, 2}, map[string]int{"z": 1}}}
	n = 8
	fmt.Println(mixed[0][0], mixed[0][1], *(mixed[0][2].(*int)), mixed[1][0], mixed[1][1])

	rows[1] = append(rows[1], "more")
	fmt.Println(rows[1]...)
	for _, row := range rows {
		for _, v := range row {
			fmt.Print(v, ";")
		}
	}
	fmt.Println()
}
