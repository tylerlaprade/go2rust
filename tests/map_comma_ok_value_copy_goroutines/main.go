package main

import "fmt"

type Point struct{ X int }

type Registry struct {
	byName map[string]Point
	counts map[string]int
}

type Scores map[string]int

// "v, ok := m[k]" gives v its own copy of the value. Writing to v afterwards
// must not change the map, and reading into existing variables must not take
// the value out of the map.
func main() {
	// A goroutine makes every variable a locked one.
	done := make(chan string, 1)
	go func() { done <- "started" }()
	fmt.Println(<-done)

	counts := map[string]int{"a": 1}
	n, ok := counts["a"]
	n = 5
	fmt.Println("int", n, ok, counts["a"])

	names := map[int]string{1: "one"}
	name, ok := names[1]
	name += "!"
	fmt.Println("string", name, ok, names[1])

	if v, ok := counts["a"]; ok {
		v++
		fmt.Println("in an if", v, counts["a"])
	}

	var m int
	var found bool
	m, found = counts["a"]
	m += 10
	fmt.Println("int into variables", m, found, counts["a"], len(counts))

	name, ok = names[1]
	fmt.Println("string into variables", name, ok, names[1])
	name, ok = names[2]
	fmt.Println("missing key into variables", name == "", ok, names[1])

	points := map[string]Point{"a": {1}}
	p, ok := points["a"]
	p.X = 9
	fmt.Println("struct", ok, points["a"].X, p.X)

	var q Point
	q, found = points["a"]
	q.X = 8
	fmt.Println("struct into variables", found, points["a"].X, q.X)

	if r, ok := points["a"]; ok {
		r.X = 7
		fmt.Println("struct in an if", points["a"].X, r.X)
	}

	missing, ok := points["zzz"]
	missing.X = 5
	fmt.Println("struct, missing key", ok, missing.X, len(points))

	reg := Registry{byName: map[string]Point{"a": {1}}, counts: map[string]int{"a": 1}}
	rp, ok := reg.byName["a"]
	rp.X = 7
	c, ok2 := reg.counts["a"]
	c++
	fmt.Println("map in a field", ok, ok2, reg.byName["a"].X, rp.X, reg.counts["a"], c)

	var c2 int
	c2, ok = reg.counts["a"]
	c2 += 5
	fmt.Println("map in a field into variables", ok, reg.counts["a"], c2)

	sc := Scores{"a": 1}
	v, ok := sc["a"]
	v = 3
	fmt.Println("named map type", ok, sc["a"], v)

	_, has := sc["zzz"]
	z, _ := sc["a"]
	z = 4
	fmt.Println("blank results", has, sc["a"], z)

	nested := map[string]map[string]int{"a": {"x": 1}}
	x, ok := nested["a"]["x"]
	x = 99
	fmt.Println("nested lookup", ok, nested["a"]["x"], x)

	// A pointer, a slice and a map are shared with the map entry, as in Go.
	ptrs := map[string]*Point{"a": {1}}
	pp, ok := ptrs["a"]
	pp.X = 9
	fmt.Println("pointer value", ok, ptrs["a"].X, pp.X)

	var pq *Point
	pq, ok = ptrs["a"]
	pq.X = 10
	fmt.Println("pointer value into variables", ok, ptrs["a"].X, pq.X)

	lists := map[string][]int{"a": {1, 2}}
	l, ok := lists["a"]
	l[0] = 50
	fmt.Println("slice value", ok, lists["a"], l)

	var l2 []int
	l2, ok = lists["a"]
	l2[1] = 60
	fmt.Println("slice value into variables", ok, lists["a"], l2)

	inner, ok := nested["a"]
	inner["x"] = 2
	fmt.Println("map value", ok, nested["a"]["x"], inner["x"])

	var inner2 map[string]int
	inner2, ok = nested["a"]
	inner2["x"] = 3
	fmt.Println("map value into variables", ok, nested["a"]["x"], inner2["x"])
}
