// The same program with a goroutine, so every value is behind a lock.
// An interface value appended to a []interface{} is copied into the slice,
// as in Go: changing the variable afterwards does not change the element.
// This did not compile: the element was copied with clone(), which a boxed
// interface value does not have.
package main

import "fmt"

type pair struct{ k string }

func main() {
	done := make(chan bool)
	go func() { done <- true }()
	<-done

	var kept []any
	var x any = "s"
	kept = append(kept, x)
	var y any = 3
	kept = append(kept, y, x)
	var p any = pair{"k"}
	kept = append(kept, p)
	y = 4
	fmt.Println(kept...)
	fmt.Println(len(kept), y)
	m := map[string]any{"a": 1}
	kept = append(kept, m["a"])
	fmt.Println(len(kept))
}
