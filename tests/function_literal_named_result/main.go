package main

import "fmt"

func main() {
	build := func() (value string) {
		value = "named result"
		return value
	}
	fmt.Println(build())
}
