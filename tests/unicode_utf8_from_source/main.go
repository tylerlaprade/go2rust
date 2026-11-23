// unicode/utf8 is translated from its Go source: decoding, encoding,
// counting and validation follow Go's own code and tables.
package main

import (
	"fmt"
	"unicode/utf8"
)

func main() {
	for _, s := range []string{"☺", "é", "a", "漢字", "𝄞x"} {
		r, size := utf8.DecodeRuneInString(s)
		fmt.Println(r, size, utf8.RuneLen(r), utf8.ValidString(s), utf8.RuneCountInString(s))
	}
	b := utf8.AppendRune(nil, 0x263A)
	fmt.Println(b, utf8.FullRune(b[:2]), utf8.Valid(b), utf8.RuneCount(b))
	r, n := utf8.DecodeLastRuneInString("x☺")
	fmt.Println(r, n, utf8.ValidRune(0xD800), utf8.ValidRune('☺'))
	buf := make([]byte, 4)
	fmt.Println(utf8.EncodeRune(buf, '𝄞'), buf, utf8.RuneStart(buf[1]))
}
