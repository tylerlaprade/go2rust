package main

import "fmt"

type nodeSet map[string]bool

type graphNode struct {
	succ nodeSet
}

func main() {
	var n graphNode
	fmt.Println(len(n.succ))
}
