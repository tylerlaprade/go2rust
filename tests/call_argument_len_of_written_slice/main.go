// len(s) as an argument of a call that writes to s: the length is read
// before the call, so the callee can write to the slice.
package main

import "fmt"

func fill(s []int, n int) {
	for i := range s {
		s[i] = n
	}
}

func shift(s []int, r int) {
	fill(s, len(s)-r)
}

func count(m map[string]int, n int) {
	m["n"] = n
}

func main() {
	s := []int{1, 2, 3}
	fill(s, len(s))
	fmt.Println(s)
	shift(s, 1)
	fmt.Println(s)
	m := map[string]int{"a": 1}
	count(m, len(m))
	fmt.Println(m)
}
