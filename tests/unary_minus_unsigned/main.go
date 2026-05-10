package main

import "fmt"

func main() {
	var n byte = 250
	v := -n
	w := -(n &^ 1) >> 1

	var small uint16 = 2
	fmt.Println(v, w, -small)
}
