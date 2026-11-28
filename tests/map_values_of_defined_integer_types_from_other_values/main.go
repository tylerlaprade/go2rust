package main

import "fmt"

type tally int

type row struct{ T tally }

func main() {
	ch := make(chan tally, 1)
	ch <- 7
	m := map[string]tally{}
	m["recv"] = <-ch
	r := row{T: 3}
	m["field"] = r.T
	s := []tally{4, 5}
	m["elem"] = s[1]
	m2 := map[string]tally{"x": 6}
	m["lookup"] = m2["x"]
	x := 8
	m["conv"] = tally(x)
	m["neg"] = -r.T
	var p *tally = &s[0]
	m["deref"] = *p
	fmt.Println(m)
}
