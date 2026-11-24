// A local variable may have the name of a package constant; inside its scope
// the name is the variable, also in an expression go/types does not fold.
package main

import "fmt"

const (
	s1   = 0x02
	mask = 0x3f
)

func decode(b0, b1 byte) rune {
	s1 := b1
	return rune(b0&0x1f)<<6 | rune(s1&mask)
}

func pick(s1 int) int {
	return s1*10 + mask
}

func main() {
	fmt.Println(decode(0xc3, 0xa9), pick(4), s1)
}
