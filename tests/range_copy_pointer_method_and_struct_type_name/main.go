package main

import "fmt"

type Vec struct{ X, Y int }

func (v *Vec) Scale(k int) { v.X *= k; v.Y *= k }

type Base struct{ ID int }

func main() {
	vs := []Vec{{1, 0}, {0, 1}}
	for i := range vs {
		vs[i].Scale(3)
	}
	// The loop variable is a copy: scaling it leaves vs alone.
	scaled := 0
	for _, v := range vs {
		v.Scale(100)
		scaled += v.X + v.Y
	}
	fmt.Println(vs, scaled)

	people := []struct {
		Name string
		Age  int
	}{{"Al", 30}}
	tagged := struct {
		Base
		Note string `json:"note"`
	}{Base{1}, "x"}
	var empty struct{}
	grid := map[string][]struct{ X, Y float64 }{}
	fmt.Printf("%T\n%T\n%T\n%T\n", people, tagged, empty, grid)
}
