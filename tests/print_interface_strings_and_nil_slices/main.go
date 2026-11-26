// fmt.Print and fmt.Sprint put a space between two operands only when
// neither is a string, and an interface value holding a string counts as
// one. A map value that is a nil slice prints as [], as other nil slices do.
package main

import (
	"fmt"
	"os"
)

type S struct{ V any }

type T struct {
	S []int
	M map[string]int
	P *int
}

func main() {
	var x any = 1
	var y any = "s"
	var z any = 2.5
	var n any
	s := S{V: "in"}
	fmt.Println(fmt.Sprint(x, y), fmt.Sprint(x, z), fmt.Sprint(y, y), fmt.Sprint(x, n, x))
	fmt.Println(fmt.Sprint(s.V, 3), fmt.Sprint(3, s.V, 4), fmt.Sprint("a", x, "b"))
	fmt.Print(x, z, "\n")
	fmt.Print(y, x, "\n")
	fmt.Fprint(os.Stdout, x, y, z, "\n")

	m := map[string][]int{"j": nil, "k": {1}}
	mm := map[string]map[string]int{"a": nil}
	fmt.Println(m, mm)
	fmt.Println([][]int{nil, {2}})
	fmt.Println(T{}, []any{[]int(nil)})
	fmt.Printf("%v %+v\n", T{}, m)
}
