package main

import "fmt"

type item struct {
	value int
}

type maker func() *item

func main() {
	makeItem := maker(func() *item {
		return &item{value: 7}
	})
	got := makeItem()
	fmt.Println(got.value)
}
