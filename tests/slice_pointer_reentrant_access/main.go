package main

import "fmt"

type cell struct {
	value int
}

type holder struct {
	current *cell
}

var cells = []cell{{value: 1}, {value: 2}}

func (c *cell) readSibling() {
	fmt.Println("sibling", cells[1].value)
	c.value = 3
}

func attach(h *holder) {
	h.current = &cells[0]
}

func main() {
	done := make(chan bool, 1)
	go func() { done <- true }()
	<-done

	h := &holder{}
	attach(h)
	h.current.readSibling()
	fmt.Println("values", cells[0].value, cells[1].value)
}
