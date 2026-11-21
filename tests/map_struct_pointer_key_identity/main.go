package main

import "fmt"

type item struct {
	value int
}

type key struct {
	p   *item
	tag int
}

func main() {
	left := item{value: 7}
	right := item{value: 7}

	values := map[key]string{}
	values[key{p: &left, tag: 1}] = "left"
	values[key{p: &right, tag: 1}] = "right"
	values[key{p: &left, tag: 2}] = "left-two"

	fmt.Println(len(values))
	fmt.Println(values[key{p: &left, tag: 1}])
	fmt.Println(values[key{p: &right, tag: 1}])
	fmt.Println(values[key{p: &left, tag: 2}])
}
