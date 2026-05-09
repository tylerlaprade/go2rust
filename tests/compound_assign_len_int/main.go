package main

import "fmt"

func main() {
	items := []int{1, 2, 3}
	marker := "xx"
	n := 10
	n -= len(items)
	n -= len(marker)
	fmt.Println(n)
}
