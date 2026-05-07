package main

import "fmt"

type Holder struct {
	ch chan int
}

func (h *Holder) Ready() bool {
	return h.ch != nil
}

func (h *Holder) Fill() {
	h.ch = make(chan int, 2)
	h.ch <- 1
	fmt.Println(h.ch != nil, len(h.ch), cap(h.ch))
	fmt.Println(<-h.ch)
	h.ch = nil
	fmt.Println(h.ch == nil)
}

func main() {
	h := Holder{}
	fmt.Println(h.Ready())
	h.Fill()

	h2 := Holder{ch: make(chan int, 1)}
	fmt.Println(h2.Ready())
	fmt.Println(len(h2.ch))
	fmt.Println(cap(h2.ch))
	h2.ch <- 7
	fmt.Println(<-h2.ch)

	h3 := Holder{ch: nil}
	fmt.Println(h3.Ready())
}
