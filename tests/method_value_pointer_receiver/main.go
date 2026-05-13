package main

import "fmt"

type tracker struct {
	value int
}

func (t *tracker) bump() {
	t.value++
}

func makeBump(t *tracker) func() {
	return t.bump
}

func main() {
	t := &tracker{}
	first := makeBump(t)
	first()
	var second func()
	second = t.bump
	second()
	go func() {}()
	fmt.Println(t.value)
}
