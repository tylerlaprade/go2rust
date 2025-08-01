package main

import "fmt"

func main() {
	i := 2
	switch i {
	case 1:
		fmt.Println("one")
	case 2:
		fmt.Println("two")
	case 3:
		fmt.Println("three")
	}

	switch {
	case i < 2:
		fmt.Println("less than 2")
	case i > 2:
		fmt.Println("greater than 2")
	default:
		fmt.Println("equal to 2")
	}
}
