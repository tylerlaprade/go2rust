package main

import (
	"fmt"
	"strings"
)

func build(done chan bool) string {
	var output strings.Builder
	output.WriteString("ready")
	done <- true
	return output.String()
}

func main() {
	done := make(chan bool, 1)
	fmt.Println(build(done))
	fmt.Println(<-done)
}
