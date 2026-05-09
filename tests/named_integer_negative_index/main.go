package main

import "fmt"

type Code int

const Invalid Code = -1

func main() {
	var x [1]struct{}
	_ = x[Invalid - -1]
	fmt.Println("ok")
}
