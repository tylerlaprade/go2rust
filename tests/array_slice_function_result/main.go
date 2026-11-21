package main

import "fmt"

func makeArray() [3]int {
	return [3]int{1, 2, 3}
}

type named [3]int

func main() {
	a := makeArray()
	view := a[:]
	view[0] = 9

	src := [3]int{4, 5, 6}
	copyArray := src
	copyView := copyArray[:]
	copyView[1] = 8

	var explicit [3]int = makeArray()
	explicitView := explicit[:]
	explicitView[2] = 7

	namedSource := named{13, 14, 15}
	var fromNamed [3]int = namedSource
	fromNamedView := fromNamed[:]
	fromNamedView[0] = 6

	fmt.Println(a, src, copyArray, explicit, namedSource, fromNamed)
}
