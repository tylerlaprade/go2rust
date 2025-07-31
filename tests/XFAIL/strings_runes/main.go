package main

import "fmt"

func main() {
	s := "hello"
	fmt.Println(len(s))

	for i := 0; i < len(s); i++ {
		fmt.Printf("%c ", s[i])
	}
	fmt.Println()

	for _, r := range "go" {
		fmt.Printf("%c ", r)
	}
	fmt.Println()
}
