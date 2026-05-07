package main

import "fmt"

func trimParens(s string) string {
	return s[len("(") : len(s)-len(")")]
}

func main() {
	fmt.Println(trimParens("(go2rust)"))
}
