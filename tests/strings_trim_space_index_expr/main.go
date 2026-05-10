package main

import (
	"fmt"
	"strings"
)

func main() {
	split := strings.Split("  file.go:12", ":")
	filename := strings.TrimSpace(split[0])
	fmt.Println(filename)
}
