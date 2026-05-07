package main

import "fmt"

func isDigit(c byte) bool {
	return '0' <= c && c <= '9'
}

func startsWithV(s string) bool {
	return len(s) > 0 && s[0] == 'v'
}

func main() {
	fmt.Println("digit 5:", isDigit('5'))
	fmt.Println("digit x:", isDigit('x'))
	fmt.Println("version v1:", startsWithV("v1.0.0"))
	fmt.Println("version x1:", startsWithV("x1.0.0"))
}
