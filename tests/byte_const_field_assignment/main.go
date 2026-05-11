package main

import "fmt"

type node struct {
	color uint8
}

func main() {
	const (
		white = 0
		grey  = 1
		black = 2
	)
	n := node{color: white}
	if n.color == white {
		n.color = grey
	}
	fmt.Println(n.color == grey)
	n.color = black
	fmt.Println(n.color)
}
