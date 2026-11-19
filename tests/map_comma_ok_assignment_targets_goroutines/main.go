package main

import "fmt"

type Numbers []int

type Index map[string]int

type Holder struct {
	nums  Numbers
	plain []int
}

var global []int

// "v, ok = m[k]" into an existing target must leave the value in the map,
// whatever the target is: a named slice, a field, a package variable, an
// element, or a variable captured by a closure.
func main() {
	done := make(chan string, 1)
	go func() { done <- "started" }()
	fmt.Println(<-done)

	m := make(map[string]Numbers)
	src := make(Numbers, 2)
	src[0], src[1] = 1, 2
	m["a"] = src

	var numbers Numbers
	var ok bool
	numbers, ok = m["a"]
	numbers[0] = 9
	fmt.Println("named slice", ok, m["a"][0], numbers[0])
	numbers = append(numbers, 5)
	fmt.Println("named slice, appended to", len(m["a"]), len(numbers))

	var h Holder
	h.nums, ok = m["a"]
	h.nums[1] = 8
	fmt.Println("named slice field", ok, m["a"][1], h.nums[1])

	lists := map[string][]int{"a": {1, 2}}
	global, ok = lists["a"]
	fmt.Println("global", ok, lists["a"], len(global), global[0], global[1])

	dst := [][]int{{9}}
	dst[0], ok = lists["a"]
	fmt.Println("indexed", ok, lists["a"], dst[0])

	h.plain, ok = lists["a"]
	h.plain[0] = 70
	fmt.Println("plain slice field", ok, lists["a"], h.plain)

	var captured []int
	set := func() bool {
		var found bool
		captured, found = lists["a"]
		return found
	}
	fmt.Println("captured by a closure", set(), lists["a"], captured)

	var capturedNamed Numbers
	setNamed := func() bool {
		var found bool
		capturedNamed, found = m["a"]
		return found
	}
	fmt.Println("named slice captured by a closure", setNamed(), len(m["a"]), len(capturedNamed))

	indexes := map[string]Index{"a": {"x": 1}}
	var idx Index
	idx, ok = indexes["a"]
	idx["x"] = 2
	fmt.Println("named map", ok, indexes["a"]["x"], idx["x"])
}
