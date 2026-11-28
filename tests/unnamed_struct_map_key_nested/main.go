package main

import "fmt"

func main() {
	m := map[struct{ p struct{ x int } }]bool{}
	k := struct{ p struct{ x int } }{}
	k.p.x = 2
	m[k] = true
	m[struct{ p struct{ x int } }{}] = false
	fmt.Println(len(m), m[k], k.p.x)
	seen := map[struct {
		name string
		at   struct{ line, col int }
	}]int{}
	seen[struct {
		name string
		at   struct{ line, col int }
	}{"a", struct{ line, col int }{1, 2}}]++
	for key, n := range seen {
		fmt.Println(key.name, key.at.line, key.at.col, n)
	}
}
