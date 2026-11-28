package main

import "fmt"

func main() {
	var x any = [][]any{{1, "a"}, {}, {nil, 2.5}}
	fmt.Println(x)
	fmt.Printf("%v %T\n", x, x)
	var y any = [][][]any{{{1}, {"b"}}}
	fmt.Println(y)
	vals := []any{[][]any{{true}}, 3}
	fmt.Println(vals)
	var nilRows [][]any
	var z any = nilRows
	fmt.Println(z)
	if rows, ok := x.([][]any); ok {
		fmt.Println(len(rows), rows[0][1])
	}
}
