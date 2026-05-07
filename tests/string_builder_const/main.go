package main

import (
	"fmt"
	"strings"
)

const separator = " | "

func main() {
	var b strings.Builder
	b.WriteString("left")
	b.WriteString(separator)
	b.WriteString("right")
	fmt.Println(b.String())
}
