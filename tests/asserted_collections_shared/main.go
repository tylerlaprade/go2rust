// A slice or map taken out of an interface{} shares its data with the one
// put in, as in Go: "m := x.(map[string]int); m["b"] = 2" changes the map x
// holds. A new variable used to get a copy, so the change was lost. Using an
// asserted map or slice directly, as in len(x.(map[string]int)) or
// x.([]int)[0], did not compile.
package main

import "fmt"

func main() {
	var counts any = map[string]int{"a": 1}
	m := counts.(map[string]int)
	m["b"] = 2
	m["a"]++
	fmt.Println(len(counts.(map[string]int)), counts.(map[string]int)["a"])

	var c = counts.(map[string]int)
	c["x"] = 7
	fmt.Println(counts.(map[string]int)["x"])

	func() {
		inner := counts.(map[string]int)
		inner["y"] = 5
	}()
	fmt.Println(len(counts.(map[string]int)))

	var list any = []int{1, 2, 3}
	items := list.([]int)
	items[0] = 10
	fmt.Println(list.([]int)[0], len(items))

	var names any = []string{"x"}
	ns := names.([]string)
	ns[0] = "z"
	fmt.Println(names.([]string))
}
