package main

import "fmt"

type entry struct {
	value any
}

type holder struct {
	table map[int][]entry
}

func main() {
	if false {
		fmt.Println(holder{})
	}
	fmt.Println("ok")
}
