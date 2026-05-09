package main

import "fmt"

type item struct {
	value int
}

type holder struct {
	ptr *item
}

func (h *holder) Store(ptr *item) {
	h.ptr = ptr
}

func main() {
	h := &holder{}
	value := item{value: 7}
	h.Store(&value)
	fmt.Println(h.ptr.value)
}
