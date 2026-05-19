package main

import (
	"fmt"
	"slices"
	"sort"
)

func compareStrings(a, b string) int {
	panic("nil slice comparator called")
}

func main() {
	var names []string
	sort.Strings(names)
	fmt.Println("strings", len(names), cap(names), names == nil)

	var nums []int
	sort.Ints(nums)
	fmt.Println("ints", len(nums), cap(nums), nums == nil)

	var ordered []int
	slices.Sort(ordered)
	fmt.Println("slices", len(ordered), cap(ordered), ordered == nil)

	var words []string
	slices.SortFunc(words, compareStrings)
	fmt.Println("sortfunc", len(words), cap(words), words == nil)
}
