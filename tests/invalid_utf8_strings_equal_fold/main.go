package main

import (
	"fmt"
	"strings"
)

type namedString string

func main() {
	badA := string([]byte{0xfa})
	badB := string([]byte{0xfb})

	fmt.Println(strings.EqualFold(badA+"A", badB+"a"))
	fmt.Println(strings.EqualFold(badA+"A", "�a"))
	fmt.Println(strings.EqualFold(badA+badB, "�"))
	fmt.Println(strings.EqualFold(string(namedString(badA+"Go")), badB+"gO"))
	fmt.Println(strings.EqualFold("Apple", "APPLE"))
	fmt.Println(strings.EqualFold("Kelvin", "kelvin"))
	fmt.Println(strings.EqualFold("ß", "ss"))

	const collision = "\ufdd0\ufdd1fa"
	fmt.Println(strings.EqualFold(collision, "\ufdd0\ufdd1FA"))
}
