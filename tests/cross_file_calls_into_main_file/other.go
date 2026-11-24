package main

import "fmt"

func other() {
	x, y := divmod(9, 4)
	fmt.Println(x, y)
	fmt.Println(divmod(7, 2))
}
