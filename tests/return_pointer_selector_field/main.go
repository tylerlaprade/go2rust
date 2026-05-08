package main

import "fmt"

type node struct {
	value int
}

type holder struct {
	child *node
}

func getChild(h *holder) *node {
	return h.child
}

func main() {
	if false {
		h := &holder{}
		fmt.Println(getChild(h))
	}
	fmt.Println("ok")
}
