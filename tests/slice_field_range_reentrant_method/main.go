package main

import "fmt"

type holder struct {
	values []string
}

func (h *holder) hasValues() bool {
	for range h.values {
		return true
	}
	return false
}

func (h *holder) countWithCheck() int {
	count := 0
	for range h.values {
		if h.hasValues() {
			count++
		}
	}
	return count
}

func main() {
	h := &holder{values: []string{"a", "b"}}
	fmt.Println(h.countWithCheck())
}
