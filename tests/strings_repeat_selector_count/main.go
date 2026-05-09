package main

import (
	"fmt"
	"strings"
)

type printer struct {
	indent int
}

func main() {
	p := printer{indent: 3}
	fmt.Println(strings.Repeat("..", p.indent) + "x")
}
