// append to a literal of a named slice type, as slices.Clone does with
// append(S{}, s...), and %T of a call whose result has a named slice type.
package main

import "fmt"

type IDs []int

func cloneIDs(s IDs) IDs {
	if s == nil {
		return nil
	}
	return append(IDs{}, s...)
}

func main() {
	ids := IDs{9, 4}
	c := append(IDs{}, ids...)
	c[0] = 1
	d := cloneIDs(ids)
	d[1] = 2
	fmt.Println(c, d, ids, cloneIDs(nil) == nil)
	fmt.Printf("%T %T %T\n", cloneIDs(ids), c, append(IDs{}, 1))
}
