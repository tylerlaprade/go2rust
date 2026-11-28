package main

import "fmt"

type Point struct{ X int }

type Inner struct{ Y int }

type Outer struct{ In Inner }

func main() {
	m := map[struct{ P Point }]bool{}
	m[struct{ P Point }{Point{1}}] = true
	m[struct{ P Point }{Point{2}}] = false
	fmt.Println(len(m), m[struct{ P Point }{Point{1}}])
	o := map[struct{ O Outer }]int{}
	o[struct{ O Outer }{Outer{Inner{3}}}]++
	fmt.Println(o)
}
