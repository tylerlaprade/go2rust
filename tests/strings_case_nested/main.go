package main

import (
	"fmt"
	"strings"
)

func clean(name string) string {
	return strings.ToUpper(strings.TrimPrefix(strings.ToLower(name), "r#"))
}

func cleanSuffix(name string) string {
	return strings.ToLower(strings.TrimSuffix(name, ".RS"))
}

func main() {
	fmt.Println(clean("R#Go2Rust"))
	fmt.Println(cleanSuffix("Go2Rust.RS"))
}
