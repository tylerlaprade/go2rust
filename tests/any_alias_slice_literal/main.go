package main

import "fmt"

func main() {
	list := []any{1, "s"}
	fmt.Println(len(list), list[0].(int), list[1].(string))
}
