package main

import "fmt"

type node interface {
	Name() string
}

type item struct {
	name string
}

func (i *item) Name() string {
	return i.name
}

type pair struct {
	left  node
	right node
}

func printPair(left, right node) {
	fmt.Println(left.Name(), right.Name())
}

func main() {
	done := make(chan bool, 1)
	go func() {
		done <- true
	}()
	<-done

	p := &pair{
		left:  &item{name: "left"},
		right: &item{name: "right"},
	}
	printPair(p.left, p.right)
}
