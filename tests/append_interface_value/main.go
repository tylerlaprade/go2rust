// An interface value appended to a []interface{} is copied into the slice,
// as in Go: changing the variable afterwards does not change the element.
// This did not compile: the element was copied with clone(), which a boxed
// interface value does not have.
package main

import "fmt"

type pair struct{ k string }

func main() {
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

	base := []any{1, 2}
	detached := append(base[:1:1], 3)
	fmt.Println(base[0], base[1], detached[0], detached[1])

	batchBase := []any{1, 2}
	batch := append(batchBase[:1:2], 3, 4)
	fmt.Println(batchBase[0], batchBase[1], batch[0], batch[1], batch[2])

	fitBase := []any{1, 2}
	fit := append(fitBase[:1:2], 3)
	fmt.Println(fitBase[1], fit[1])
}
