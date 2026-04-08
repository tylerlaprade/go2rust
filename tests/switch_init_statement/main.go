package main

import "fmt"

func main() {
	values := []int{1, 2, 3, 4}

	switch n := len(values); n {
	case 0:
		fmt.Println("empty")
	case 4:
		fmt.Println("len is four")
	default:
		fmt.Println("other")
	}

	switch x := values[1] * 10; {
	case x > 30:
		fmt.Println("large")
	default:
		fmt.Println("small")
	}
}
