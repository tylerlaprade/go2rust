package main

import (
	"fmt"
	"strings"
)

func main() {
	fmt.Println(strings.ToUpper("hello world"))
	fmt.Println(strings.ToLower("HELLO WORLD"))
	fmt.Println(strings.TrimSpace("  hello world  "))
	fmt.Println(strings.ToUpper(strings.TrimSpace("  hello  ")))
}
