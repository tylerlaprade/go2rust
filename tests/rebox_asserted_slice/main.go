// A slice or map taken out of one interface{} and put into another is still
// a slice or map there. The second assertion used to stop the program.
package main

import "fmt"

func main() {
	a := any([]int{1, 2})
	b := any(a.([]int))
	c := b.([]int)
	fmt.Println(len(c), c[0], c[1])

	var holder any = a.([]int)
	d, ok := holder.([]int)
	fmt.Println(ok, d)

	m := any(map[string]int{"k": 1})
	n := any(m.(map[string]int))
	back, ok := n.(map[string]int)
	fmt.Println(ok, back["k"], len(back))

	items := []any{a.([]int), m.(map[string]int)}
	for _, item := range items {
		switch v := item.(type) {
		case []int:
			fmt.Println("slice", len(v))
		case map[string]int:
			fmt.Println("map", len(v))
		}
	}
}
