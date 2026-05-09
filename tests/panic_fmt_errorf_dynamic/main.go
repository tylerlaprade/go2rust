package main

import "fmt"

func panicf(format string, args ...any) {
	panic(fmt.Errorf(format, args...))
}

func main() {
	fmt.Println("ok")
}
