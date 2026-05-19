package main

import "fmt"

type Settings struct {
	Enabled bool
	Name    string
	Count   int
	Ratio   float64
}

func main() {
	var zero Settings
	partial := Settings{Name: "go"}

	fmt.Println(zero.Enabled, zero.Name == "", zero.Count, zero.Ratio == 0)
	fmt.Println(partial.Enabled, partial.Name, partial.Count)
}
