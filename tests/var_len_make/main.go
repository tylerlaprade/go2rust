package main

import "fmt"

func main() {
	names := []string{"alpha", "beta", "gamma"}
	var (
		n   = len(names)
		out = make([]string, n)
	)
	out[1] = "beta"
	fmt.Println(len(out), out[1])
}
