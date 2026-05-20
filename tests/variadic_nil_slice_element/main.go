package main

import (
	"fmt"
	"strings"
)

func collect(groups ...[]string) []string {
	var out []string
	for _, group := range groups {
		for _, value := range group {
			out = append(out, value)
		}
	}
	return out
}

func main() {
	var missing []string
	values := collect([]string{"go"}, missing, []string{"rust"})
	fmt.Println(strings.Join(values, ","))
}
