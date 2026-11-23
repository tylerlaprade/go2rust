package main

import "fmt"

func main() {
	list := []interface{}{7, "x"}
	n, ok := list[0].(int)
	fmt.Println(n, ok)
	s, ok := list[1].(string)
	fmt.Println(s, ok)
	missing, ok := list[1].(int)
	fmt.Println(missing, ok)
}
