package main

import "fmt"

func seed() {
	const flag = true
	_ = flag
}

func check(flag bool) {
	if !flag {
		fmt.Println("off")
	}
}

func main() {
	seed()
	check(false)
}
