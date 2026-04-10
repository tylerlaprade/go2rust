package main

import "fmt"

func join(prefix string, parts ...string) string {
	out := prefix
	for _, p := range parts {
		out += ":" + p
	}
	return out
}

func main() {
	rest := []string{"b", "c"}
	fmt.Println(join("a", rest...))
	fmt.Println(join("start", "x", rest...))
}
