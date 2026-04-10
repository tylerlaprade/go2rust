package main

import "fmt"

func join(prefix string, sep string, parts ...string) string {
	out := prefix
	for _, p := range parts {
		out += sep + p
	}
	return out
}

func main() {
	rest := []string{"b", "c"}
	fmt.Println(join("a", ":", rest...))
}
