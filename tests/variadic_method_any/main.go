package main

import "fmt"

type logger struct{}

func (l logger) trace(format string, args ...any) {
	fmt.Println(format, len(args))
}

func main() {
	var l logger
	l.trace("objects", 1, "two")
}
