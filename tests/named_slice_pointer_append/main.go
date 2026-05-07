package main

import "fmt"

type node struct {
	value int
}

type nodes []*node

func add(xs nodes, n *node) nodes {
	xs = append(xs, n)
	return xs
}

func keep(xs nodes) nodes {
	var kept nodes
	for _, n := range xs {
		kept = append(kept, n)
	}
	return kept
}

func main() {
	var xs nodes
	n := &node{value: 7}
	xs = add(xs, n)
	fmt.Println(len(xs))
	fmt.Println(len(keep(xs)))
}
