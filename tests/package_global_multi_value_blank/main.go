package main

import "fmt"

var enabled, _ = parseEnabled()

func parseEnabled() (bool, error) {
	return true, nil
}

func main() {
	fmt.Println(enabled)
}
