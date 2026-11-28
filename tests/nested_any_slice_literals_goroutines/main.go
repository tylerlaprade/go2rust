package main

import (
	"fmt"
	"sync"
)

var table = [][]any{{"id", 1}, {"name", "x", 2.5}}

var byKey = map[string][]any{"b": {true}, "a": {1, "y"}}

func main() {
	var wg sync.WaitGroup
	var mu sync.Mutex
	rows := [][]any{}
	for i := 0; i < 3; i++ {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()
			mu.Lock()
			rows = append(rows, []any{i})
			mu.Unlock()
		}(i)
	}
	wg.Wait()
	fmt.Println(len(rows))
	fmt.Println(table, len(table[1]))
	fmt.Println(byKey)
	for _, row := range table {
		fmt.Println(row...)
	}
	grid := [2][]any{{1.5, true}, {nil}}
	fmt.Println(grid, grid[1][0] == nil)
	fmt.Printf("%v\n", table)
	s := fmt.Sprint(table)
	fmt.Println(s)
}
