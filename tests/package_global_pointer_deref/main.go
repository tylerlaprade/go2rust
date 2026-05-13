package main

import "fmt"

func boolPtr(v bool) *bool {
	return &v
}

func stringPtr(v string) *string {
	return &v
}

var enabled = boolPtr(true)
var label = stringPtr("ready")

func invert(v bool) bool {
	return !v
}

func suffix(s string) string {
	return s + "!"
}

func main() {
	fmt.Println(*enabled)
	fmt.Println(*label)
	fmt.Println(invert(*enabled))
	fmt.Println(suffix(*label))
}
