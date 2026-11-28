package main

import "fmt"

func main() {
	m := map[struct{ Q [2]int }]int{}
	m[struct{ Q [2]int }{}]++
	fmt.Println(m[struct{ Q [2]int }{Q: [2]int{}}], m[struct{ Q [2]int }{Q: [2]int{1, 0}}], len(m))
	n := map[struct {
		Q [2][1]struct{ y int }
	}]int{}
	n[struct {
		Q [2][1]struct{ y int }
	}{}]++
	var z [2][1]struct{ y int }
	fmt.Println(n[struct {
		Q [2][1]struct{ y int }
	}{Q: z}])
	scalars()
}

func scalars() {
	m := map[struct {
		x, y int
		s    string
	}]int{}
	m[struct {
		x, y int
		s    string
	}{x: 1}]++
	fmt.Println(m[struct {
		x, y int
		s    string
	}{1, 0, ""}], len(m))
}
