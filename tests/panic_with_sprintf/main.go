package main

import "fmt"

func check(n int) {
	if n < 0 {
		panic(fmt.Sprintf("invalid n %d (should be >= 0)", n))
	}
	fmt.Println("ok", n)
}

func main() {
	check(5)
}
