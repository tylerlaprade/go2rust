package main

import "fmt"

type holder struct {
	table map[int]string
}

func remove(h *holder, key int) {
	delete(h.table, key)
}

func main() {
	go func() {}()
	h := &holder{table: map[int]string{1: "one"}}
	fmt.Println(h.table[1])
	remove(h, 1)
	_, ok := h.table[1]
	fmt.Println(ok)
}
