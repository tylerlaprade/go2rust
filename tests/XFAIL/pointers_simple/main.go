package main

import "fmt"

func main() {
	x := 42
	p := &x
	fmt.Println("x:", x)
	fmt.Println("p points to:", *p)

	*p = 100
	fmt.Println("x after change:", x)
}
