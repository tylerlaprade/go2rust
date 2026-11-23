package main

import (
	"fmt"
	"os"
)

func main() {
	first, firstErr := os.Hostname()
	second, secondErr := os.Hostname()
	fmt.Println(firstErr == nil, secondErr == nil, first != "", first == second)
}
