package main

import (
	"fmt"
	"regexp"
)

func main() {
	pattern := `\d+`
	re := regexp.MustCompile(pattern)
	text := "I have 42 apples and 7 oranges"
	matches := re.FindAllString(text, -1)
	fmt.Println("Numbers found:", matches)
}
