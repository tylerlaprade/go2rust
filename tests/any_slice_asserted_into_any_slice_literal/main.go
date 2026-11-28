package main

import "fmt"

func main() {
	var x any = []any{1, "a"}
	ys := []any{x.([]any), 2}
	fmt.Println(ys, len(ys))
	z, ok := x.([]any)
	var w any = z
	fmt.Println(w, ok)
}
