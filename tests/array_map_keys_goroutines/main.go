// A map keyed by arrays: literal keys in lookups, assignments and deletes
// are the arrays themselves, and the map prints its keys as arrays. A
// goroutine makes every variable a locked one.
package main

import "fmt"

type P struct{ X, Y int }

func main() {
	done := make(chan bool)
	go func() { done <- true }()
	<-done
	arrKey := map[[2]int]int{{1, 2}: 3}
	arrKey[[2]int{4, 5}] = 9
	k := [2]int{1, 2}
	v, ok := arrKey[[2]int{4, 5}]
	delete(arrKey, [2]int{7, 7})
	fmt.Println(arrKey[[2]int{1, 2}], arrKey[k], v, ok, len(arrKey))
	seen := map[[3]string]bool{}
	seen[[3]string{"a", "b", "c"}] = true
	fmt.Println(seen[[3]string{"a", "b", "c"}], seen[[3]string{}])
	grid := map[P]string{{1, 2}: "a"}
	grid[P{3, 4}] = "b"
	fmt.Println(grid[P{1, 2}], grid[P{3, 4}], grid)
	fmt.Println(arrKey)
}
