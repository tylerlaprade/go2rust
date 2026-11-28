package main

import "fmt"

type Counts map[string]int

var groups = map[string][]map[string]int{"a": {{"x": 1}, nil}}
var table = map[string][]map[string]int{"row": {1: {"x": 1}}}

func main() {
	input := make(chan map[string]int, 1)
	output := make(chan map[string]int, 1)
	input <- map[string]int{"k": 1}
	close(input)
	for m := range input {
		output <- m
	}
	fmt.Println(len(<-output))

	values := []any{map[string]int{"v": 2}}
	ms := []map[string]int{values[0].(map[string]int)}
	fmt.Println(ms[0]["v"])

	s := []map[string]int{{"c": 3}}
	var c Counts = s[0]
	c["d"] = 4
	fmt.Println(len(c), len(s[0]))

	fmt.Println(len(groups["a"]), groups["a"][1] == nil, groups["a"][0]["x"], table["row"][0] == nil, table["row"][1]["x"])
}
