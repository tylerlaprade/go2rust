package main

import "fmt"

type I interface{ M() int }

type Base struct{}

func (Base) M() int { return 1 }

type Outer struct {
	Base
	L Leaf
}

func main() {
	var a I = Outer{L: Leaf{N: 1}}
	var b I = Outer{L: Leaf{N: 2}}
	var c I = Outer{L: Leaf{N: 1}}
	fmt.Println(a.M(), a == b, a == c)
}
