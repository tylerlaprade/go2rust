package main

import "fmt"

type item struct {
	value int
}

type key struct {
	p   *item
	tag int
}

func main() {
	left := item{value: 7}
	right := item{value: 7}

	values := map[key]string{}
	values[key{p: &left, tag: 1}] = "left"
	values[key{p: &right, tag: 1}] = "right"
	values[key{p: &left, tag: 2}] = "left-two"

	fmt.Println(len(values))
	fmt.Println(values[key{p: &left, tag: 1}])
	fmt.Println(values[key{p: &right, tag: 1}])
	fmt.Println(values[key{p: &left, tag: 2}])

	// A map assignment copies its struct key before evaluating the later
	// mutation. Reading the key on the right-hand side must not turn that Go
	// value copy into a shared Rust Clone.
	snapshots := map[key]string{}
	k := key{p: &left, tag: 3}
	snapshots[k] = fmt.Sprint(k.tag)
	k.tag = 4
	fmt.Println(len(snapshots))
	fmt.Println(snapshots[key{p: &left, tag: 3}])
	fmt.Println("missing", snapshots[key{p: &left, tag: 4}] == "")

	rangeSource := map[key]bool{key{p: &right, tag: 5}: true}
	rangeSnapshots := map[key]string{}
	for rangeKey := range rangeSource {
		rangeSnapshots[rangeKey] = fmt.Sprint(rangeKey.tag)
		rangeKey.tag = 6
	}
	fmt.Println(len(rangeSnapshots))
	fmt.Println(rangeSnapshots[key{p: &right, tag: 5}])
	fmt.Println("missing", rangeSnapshots[key{p: &right, tag: 6}] == "")
}
