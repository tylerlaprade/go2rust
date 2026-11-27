package main

import "fmt"

func main() {
	seen := map[struct{ x, y int }]bool{}
	seen[origin()] = true
	seen[struct{ x, y int }{1, 2}] = true
	fmt.Println(len(seen), seen[origin()])
}
