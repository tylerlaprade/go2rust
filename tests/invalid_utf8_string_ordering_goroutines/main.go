// The same invalid-byte ordering checks with a goroutine present, so generated
// string values use their concurrent wrappers.
package main

import "fmt"

type label string

func main() {
	go func() {}()
	invalid := string([]byte{0xfa})
	validHigh := "\U0010ffff"
	invalidLow := string([]byte{0xf9})
	named := label(invalid)

	fmt.Println(invalid > validHigh, validHigh < invalid)
	fmt.Println(invalidLow < invalid, invalid >= invalidLow)
	fmt.Println(named > label(validHigh), named == label(invalid))
}
