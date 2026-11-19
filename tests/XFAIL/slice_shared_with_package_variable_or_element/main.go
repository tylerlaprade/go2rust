package main

import "fmt"

var global []int

// A slice assigned to a package variable, or to an element of a slice of
// slices, is the same slice afterwards: a write through one name shows
// through the other. The generated code gives these two targets elements of
// their own, because a package-level slice and an element of [][]int each
// hold their elements directly instead of a handle that could be shared
// (DEV-36). The last case does not compile yet.
func main() {
	lists := map[string][]int{"a": {1, 2}}
	var ok bool
	global, ok = lists["a"]
	global[0] = 7
	fmt.Println("two results into a package variable", ok, lists["a"][0], global[0])

	dst := [][]int{{9}}
	dst[0], ok = lists["a"]
	dst[0][1] = 8
	fmt.Println("two results into an element", ok, lists["a"][1], dst[0][1])

	other := map[string][]int{"a": {1, 2}}
	global = other["a"]
	global[0] = 7
	fmt.Println("one result into a package variable", other["a"][0], global[0])

	local := []int{1, 2}
	global = local
	global[0] = 70
	fmt.Println("a local slice into a package variable", local[0], global[0])

	dst[0] = local
	dst[0][1] = 80
	fmt.Println("a local slice into an element", local[1], dst[0][1])
}
