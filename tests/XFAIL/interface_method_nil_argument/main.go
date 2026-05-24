package main

import "fmt"

type Node interface {
	NodeName() string
}

func describe(node Node) {
	if node == nil {
		fmt.Println("nil node")
		return
	}
	fmt.Println(node.NodeName())
}

func main() {
	describe(nil)
}
