package main

import "fmt"

type Row struct{ M map[string]int }

func main() {
	s := []map[string]int{{"x": 1}, {"y": 2}}
	r := Row{M: s[0]}
	r.M["z"] = 3
	fmt.Println(len(s[0]), len(r.M))
	clear(s[1:])
	fmt.Println(s[1] == nil, len(s))
	clear(s)
	fmt.Println(s[0] == nil, len(r.M))
}
