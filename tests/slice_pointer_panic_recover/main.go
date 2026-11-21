package main

import "fmt"

type cell struct {
	value int
}

type holder struct {
	current *cell
}

var cells = []cell{{value: 1}, {value: 2}}

func (c *cell) failAfterWrite() {
	c.value = 7
	panic("boom")
}

func attach(h *holder) {
	h.current = &cells[0]
}

func callAndRecover(h *holder) {
	defer func() {
		if recover() != nil {
			fmt.Println("recovered")
		}
	}()
	h.current.failAfterWrite()
}

func main() {
	done := make(chan bool, 1)
	go func() { done <- true }()
	<-done

	h := &holder{}
	attach(h)
	callAndRecover(h)
	fmt.Println("values", cells[0].value, cells[1].value)
}
