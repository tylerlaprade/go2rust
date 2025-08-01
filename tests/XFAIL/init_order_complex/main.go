package main

import "fmt"

var (
	a = c + b
	b = f()
	c = f()
	d = 3
)

func f() int {
	d++
	return d
}

func init() {
	fmt.Println("First init")
	d++
}

func init() {
	fmt.Println("Second init")
	fmt.Printf("a=%d, b=%d, c=%d, d=%d\n", a, b, c, d)
}

func main() {
	fmt.Printf("In main: a=%d, b=%d, c=%d, d=%d\n", a, b, c, d)
}
