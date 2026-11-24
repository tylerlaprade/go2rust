package main

import "fmt"

func main() {
	other()
	fmt.Println(divmod(9, 4))
}

func divmod(a, b int) (int, int) { return a / b, a % b }
