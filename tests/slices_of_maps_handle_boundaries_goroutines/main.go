package main

import (
	"fmt"
	"sync"
)

func first(s []map[string]int) map[string]int {
	return s[0]
}

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
	s := []map[string]int{{"x": 1}}
	var m = s[0]
	m["y"] = 2
	r := first(s)
	r["z"] = 3
	fmt.Println(len(s[0]), s[0]["y"], len(m), len(r))
	n := [][]map[string]int{{{"x": 1}}}
	fmt.Println(n[0][0]["x"])
	n[0][0]["w"] = 5
	v, ok := n[0][0]["w"]
	fmt.Println(v, ok, len(n[0][0]))
	var p map[string]int
	p = s[0]
	p["q"] = 4
	fmt.Println(len(s[0]))
}
