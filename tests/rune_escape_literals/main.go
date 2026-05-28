package main

import "fmt"

// Go rune/char literals using Go-only escapes (\a \b \f \v) and the 8-digit
// \U form, which Rust char literals reject. Must translate to \u{..} at every
// emission site (expression, const declaration, byte cast).
const bell = '\a'
const maxRune = '\U0010FFFF'

func main() {
	v := '\v'
	b := '\b'
	f := byte('\f')
	fmt.Println(int(v), int(bell), int(maxRune), int(b), f)
}
