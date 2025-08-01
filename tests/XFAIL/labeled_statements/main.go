package main

import "fmt"

func main() {
outer:
	for i := 0; i < 3; i++ {
		for j := 0; j < 3; j++ {
			if i == 1 && j == 1 {
				continue outer
			}
			if i == 2 && j == 1 {
				break outer
			}
			fmt.Printf("(%d, %d) ", i, j)
		}
		fmt.Println()
	}
	fmt.Println("Done")
}
