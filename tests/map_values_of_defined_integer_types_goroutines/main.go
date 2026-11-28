package main

import (
	"fmt"
	"sync"
)

type tally int

type level uint8

const big tally = 40

func next(t tally) tally { return t + 1 }

func main() {
	var wg sync.WaitGroup
	wg.Add(1)
	go func() {
		defer wg.Done()
		run()
	}()
	wg.Wait()
}

type row struct{ T tally }

func run() {
	s := []tally{0, 0}
	s[1] = 5
	r := row{}
	r.T = 6
	fmt.Println(s, r.T)
	var t tally = 4
	m := map[string]tally{}
	m["a"] = 3
	m["b"] = big
	m["c"] = t + 1
	m["d"] = t
	t = 9
	m["e"] = next(t)
	m["e"] += 2
	m["e"]++
	m["f"] = m["e"] * 2
	lv := map[int]level{1: 2}
	lv[2] = 7
	lv[1] |= 4
	type pair struct{ m map[string]tally }
	p := pair{m: map[string]tally{}}
	p.m["x"] = 9
	fmt.Println(m, lv, p.m["x"], t)
}
