package main

import "fmt"

func describe(v interface{}) {
	fmt.Printf("%v\n", v)
}

func main() {
	list := []interface{}{7, "x"}
	for _, item := range list {
		describe(item)
	}
	describe(list[0])
}
