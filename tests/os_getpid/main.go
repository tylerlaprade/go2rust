package main

import (
	"fmt"
	"os"
)

func main() {
	first := os.Getpid()
	second := os.Getpid()
	fmt.Println(first > 0, first == second)
}
