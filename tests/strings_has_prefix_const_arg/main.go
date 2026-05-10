package main

import (
	"fmt"
	"strings"
)

const marker = "$"

func main() {
	name := "$1"
	fmt.Println(strings.HasPrefix(name, marker))
}
