package main

import (
	"fmt"
	"strings"
)

func wrap(name string) string {
	var out strings.Builder
	out.WriteString("(" + name + ")")
	return out.String()
}

func main() {
	fmt.Println(wrap("gopher"))
}
