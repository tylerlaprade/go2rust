package main

import (
	"fmt"
	"sync"
)

type Tables []map[string]int

type row struct{ M map[string]int }

type grid struct {
	A [33]map[string]int
	S []map[string]int
}

func first(s []map[string]int) map[string]int { return (s[0]) }

func main() {
	var wg sync.WaitGroup
	wg.Add(1)
	go func() {
		defer wg.Done()
		run()
	}()
	wg.Wait()
}

func run() {
	ch := make(chan map[string]int, 3)
	ch <- map[string]int{"a": 1}
	ch <- map[string]int{"b": 2}
	ch <- map[string]int{"c": 3}
	r := []map[string]int{<-ch}
	r = append(r, <-ch)
	r[0] = <-ch
	fmt.Println(len(r), len(r[0]), r[0]["c"], r[1]["b"])

	t := make(Tables, 2)
	fmt.Println(len(t), t[0] == nil)
	t[1] = map[string]int{"a": 1}
	fmt.Println(len(t[1]))

	rows := []row{{}}
	m := map[string]int{"k": 1}
	rows[0].M = m
	m["k2"] = 2
	fmt.Println(len(rows[0].M))
	rows[0].M = nil
	fmt.Println(rows[0].M == nil, len(m))

	s := []map[string]int{{"c": 3}}
	p, n := s[0], 0
	p["z"] = 9
	fmt.Println(len(s[0]), n, len(first(s)))

	var a [33]map[string]int
	a[0] = map[string]int{"x": 1}
	fmt.Println(a)
	var g grid
	g.A[1] = map[string]int{"y": 2}
	g.S = []map[string]int{nil, {"w": 4}}
	fmt.Println(g)

	nilMaps := make([]map[string]int, 1)
	func() {
		defer func() { fmt.Println("recovered:", recover()) }()
		alias := nilMaps[0]
		alias["boom"] = 1
	}()
}
