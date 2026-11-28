package main

import "fmt"

func main() {
	r0 := []any{1, "a"}
	r1 := []any{2}
	rows := [][]any{r0, r1}
	values := []any{rows[0], rows[1]}
	fmt.Println(values, len(values))
	var one any = rows[0]
	fmt.Println(one)
}
