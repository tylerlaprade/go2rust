package main

import "fmt"

type item struct {
	value int
}

func first(items []*item) *item {
	return items[0]
}

func main() {
	ptr := &item{value: 4}
	items := []*item{ptr}
	got := first(items)

	ptr.value = 9
	fmt.Println(got.value)
}
