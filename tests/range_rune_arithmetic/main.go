// Arithmetic on the runes of a range over a string: digit values, shifts
// within the alphabet, and runes built back from them.
package main

import (
	"fmt"
	"strings"
)

func atoi(s string) int {
	n := 0
	for _, c := range s {
		n = n*10 + int(c-'0')
	}
	return n
}

func caesar(s string, k int) string {
	var b strings.Builder
	for _, c := range s {
		switch {
		case c >= 'a' && c <= 'z':
			b.WriteRune('a' + (c-'a'+rune(k))%26)
		case c >= 'A' && c <= 'Z':
			b.WriteRune('A' + (c-'A'+rune(k))%26)
		default:
			b.WriteRune(c)
		}
	}
	return b.String()
}

func main() {
	fmt.Println(atoi("4096") + 1)
	fmt.Println(caesar("Hello, World!", 3), caesar("xyz", 2))
	counts := [26]int{}
	for _, c := range "banana" {
		counts[c-'a']++
	}
	fmt.Println(counts[0], counts[1], counts[13])
	for i, c := range "az" {
		up := c - 'a' + 'A'
		next := c + 1
		fmt.Println(i, string(up), string(next), c*2, c|0x20, c&^0x20)
	}
}
