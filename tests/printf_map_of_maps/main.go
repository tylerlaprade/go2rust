// A map whose values are maps, map[string]map[string]int: the inner map has
// no Display, so format_map of the outer map did not compile. Each inner
// map is now printed by format_map of its own cell.
package main

import "fmt"

func main() {
	nested := map[string]map[string]int{"a": {"x": 1, "y": 2}, "b": nil}
	nested["c"] = map[string]int{}
	nested["c"]["z"] = 9
	fmt.Println(nested, len(nested))
	byID := map[int]map[string]bool{2: {"t": true}, 1: {"f": false}}
	fmt.Println(byID)
	fmt.Printf("%v\n", nested)
}
