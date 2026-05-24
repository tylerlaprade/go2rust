package main

import "fmt"

type Pos int

func nextPos(p Pos) Pos {
	return p + 1
}

func sumPos(a Pos, b Pos) Pos {
	return a + b
}

func diffPos(a Pos, b Pos) Pos {
	return a - b
}

func main() {
	var a Pos = 10
	var b Pos = 5
	fmt.Println(nextPos(a))
	fmt.Println(sumPos(a, b))
	fmt.Println(diffPos(a, b))
}
