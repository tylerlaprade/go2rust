package main

import "fmt"

func divmod(a, b int) (int, int) { return a / b, a % b }

func main() {
	var (
		a, b int = 1, 2
		s, t     = "go", "rust"
		q, r     = divmod(17, 5)
	)
	fmt.Println(a, b, s, t, q, r)
}
