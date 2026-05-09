package main

import "fmt"

type item struct {
	value int
}

func main() {
	ptr := &item{value: 4}
	items := []*item{ptr}

	ptr.value = 9
	fmt.Println(items[0].value)
}
