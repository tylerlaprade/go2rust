package main

import (
	"fmt"
	"strings"
)

func classify(value string) string {
	switch strings.ToLower(value) {
	case "go", "rust":
		return "systems"
	case "python":
		return "scripting"
	default:
		return "other"
	}
}

func main() {
	fmt.Println(classify("Go"))
	fmt.Println(classify("python"))
	fmt.Println(classify("zig"))
}
