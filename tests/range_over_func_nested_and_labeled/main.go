// Ranges over functions inside other loops: a labeled continue or break of
// the loop around one, a range over a function inside another with a
// return of results from the inner one, and for k, v = range seq.
package main

import "fmt"

func count(n int) func(func(int) bool) {
	return func(yield func(int) bool) {
		for i := 0; i < n; i++ {
			if !yield(i) {
				fmt.Println("  stopped at", i)
				return
			}
		}
	}
}

func pairs(yield func(string, int) bool) {
	for i, s := range []string{"a", "b", "c"} {
		if !yield(s, i) {
			return
		}
	}
}

func findPair(target int) (name string, found bool) {
	for s, i := range pairs {
		for j := range count(3) {
			if i+j == target {
				name = fmt.Sprintf("%s%d", s, j)
				return name, true
			}
		}
	}
	return
}

func firstOver(limit int) int {
	for i := range count(10) {
		for j := range count(10) {
			if i*j > limit {
				return i*10 + j
			}
		}
	}
	return -1
}

func main() {
outer:
	for _, row := range []int{1, 2, 3} {
		for v := range count(5) {
			if v == row {
				continue outer
			}
			if row == 3 && v == 1 {
				break outer
			}
			fmt.Println("row", row, "v", v)
		}
		fmt.Println("end row", row)
	}

	var s string
	var k int
	for s, k = range pairs {
		if k == 1 {
			break
		}
	}
	fmt.Println("last", s, k)

loop:
	for a := range count(3) {
		for b := range count(3) {
			if b > a {
				continue loop
			}
			if a == 2 && b == 1 {
				break loop
			}
			fmt.Println("a", a, "b", b)
		}
	}

	fmt.Println(findPair(3))
	fmt.Println(findPair(9))
	fmt.Println(firstOver(20))
	total := 0
	for range count(4) {
		total++
	}
	fmt.Println("total", total)
}
