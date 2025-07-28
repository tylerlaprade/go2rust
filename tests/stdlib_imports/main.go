package main

import (
	"fmt"
	"strconv"
	"strings"
)

func main() {
	fmt.Println("Testing multiple imports")

	upper := strings.ToUpper("hello")
	fmt.Println("Upper:", upper)

	num := 42
	str := strconv.Itoa(num)
	fmt.Println("Number as string:", str)

	lower := strings.ToLower("WORLD")
	fmt.Println("Lower:", lower)
}
