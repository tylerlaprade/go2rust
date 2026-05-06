package main

import "fmt"

func classify(n int) string {
	if n < 0 {
		return "negative"
	} else if x := n; x == 4 {
		return "four"
	} else if y := n; y == 9 {
		return "nine"
	}
	return "other"
}

func main() {
	fmt.Println(classify(4))
	fmt.Println(classify(9))
	fmt.Println(classify(5))
}
