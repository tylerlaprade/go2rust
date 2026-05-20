package main

import (
	"fmt"
	"strings"
)

type source struct {
	files []string
}

func flatten(groups [][]string) []string {
	var out []string
	for _, group := range groups {
		out = append(out, group...)
	}
	return out
}

func main() {
	var src source
	groups := [][]string{src.files, {"go"}}
	fmt.Println(len(groups[0]))
	fmt.Println(strings.Join(flatten(groups), ","))
}
