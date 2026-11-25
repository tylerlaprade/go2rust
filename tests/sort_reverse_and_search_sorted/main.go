package main

import (
	"fmt"
	"sort"
)

type byLen []string

func (b byLen) Len() int           { return len(b) }
func (b byLen) Less(i, j int) bool { return len(b[i]) < len(b[j]) }
func (b byLen) Swap(i, j int)      { b[i], b[j] = b[j], b[i] }

type point struct{ x, y int }

type byX []point

func (p byX) Len() int { return len(p) }

// Less reads two slice elements in one expression; the method body is
// a single tail expression that starts with a block.
func (p byX) Less(i, j int) bool { return p[i].x < p[j].x }
func (p byX) Swap(i, j int)      { p[i], p[j] = p[j], p[i] }

func firstAtLeast(values []int, limit int) int {
	return sort.Search(len(values), func(i int) bool { return values[i] >= limit })
}

func main() {
	words := []string{"banana", "kiwi", "apple", "fig"}
	sort.Sort(byLen(words))
	fmt.Println(words, sort.IsSorted(byLen(words)))
	sort.Stable(sort.Reverse(byLen(words)))
	fmt.Println(words, sort.IsSorted(byLen(words)), sort.IsSorted(sort.Reverse(byLen(words))))

	points := []point{{3, 1}, {1, 2}, {2, 3}, {1, 0}}
	sort.Stable(byX(points))
	fmt.Println(points)
	sort.Sort(sort.Reverse(byX(points)))
	fmt.Println(points[0], points[1])

	ints := []int{1, 3, 5, 7}
	fmt.Println(sort.SearchInts(ints, 5), sort.SearchInts(ints, 4), sort.SearchInts(ints, 9), sort.SearchInts(nil, 1))
	floats := []float64{0.5, 1.5, 2.5}
	fmt.Println(sort.SearchFloat64s(floats, 1.5), sort.SearchFloat64s(floats, 3))
	names := []string{"ada", "bob", "eve"}
	fmt.Println(sort.SearchStrings(names, "bob"), sort.SearchStrings(names, "carl"))

	fmt.Println(sort.Search(100, func(i int) bool { return i*i >= 50 }), firstAtLeast(ints, 6))
}
