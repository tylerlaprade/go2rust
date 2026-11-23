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
}
