// strconv.Atoi moved a string variable it was given, so a program that used
// the string again after parsing it, as in an error message, did not
// compile.
package main

import (
	"fmt"
	"strconv"
	"strings"
)

func main() {
	total := 0
	for _, field := range strings.Split("1, 2,x, 30", ",") {
		field = strings.TrimSpace(field)
		n, err := strconv.Atoi(field)
		if err != nil {
			fmt.Println("skip", field, err)
			continue
		}
		total += n
		fmt.Println(field, "->", n)
	}
	for _, word := range []string{"true", "nope"} {
		b, err := strconv.ParseBool(word)
		fmt.Println(word, b, err)
	}
	fmt.Println(total)
}
