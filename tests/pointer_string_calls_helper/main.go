package main

import "fmt"

type holder struct {
	value string
}

func (h *holder) toString() string {
	return h.value
}

func (h *holder) String() string {
	return h.toString()
}

func main() {
	go func() {}()

	h := &holder{value: "ok"}
	fmt.Println(h.String())
}
