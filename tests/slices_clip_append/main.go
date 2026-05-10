package main

import (
	"fmt"
	"slices"
)

func main() {
	env := make([]string, 1, 2)
	env[0] = "A=1"

	combined := append(slices.Clip(env), "PWD=/tmp")
	combined[0] = "B=2"

	fmt.Println(env)
	fmt.Println(combined)
}
