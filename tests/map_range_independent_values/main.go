package main

import "fmt"

type Point struct{ X int }

func (p *Point) Inc() { p.X++ }

func main() {
	numbers := map[string]int{"a": 1}
	for _, v := range numbers {
		v = 5
		fmt.Println("int", v, numbers["a"])
	}
	sum := 0
	max := 0
	for _, v := range numbers {
		sum += v
		if v > max {
			max = v
		}
	}
	fmt.Println("aggregate", sum, max)

	weights := map[string]float64{"a": 1.5, "b": 2.25}
	totalWeight := 0.0
	for _, weight := range weights {
		totalWeight += weight
	}
	fmt.Println("weight", totalWeight)

	words := map[string]string{"a": "one"}
	for _, word := range words {
		word += "!"
		fmt.Println("string", word, words["a"])
	}

	points := map[string]Point{"a": {X: 1}}
	for _, p := range points {
		p.Inc()
		fmt.Println("struct", p.X, points["a"].X)
	}

	lists := map[string][]int{"a": {1}}
	for _, list := range lists {
		list[0] = 2
	}
	fmt.Println("slice", lists["a"][0])

	n := 3
	ptrs := map[string]*int{"a": &n}
	for _, p := range ptrs {
		*p = 4
	}
	fmt.Println("pointer", n)

	nilPtrs := map[string]*int{"a": nil}
	for _, p := range nilPtrs {
		fmt.Println("nil-pointer", p == nil)
	}
}
