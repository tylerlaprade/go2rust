package main

import "fmt"

const debug = false

func main() {
	if debug {
		fmt.Println("debug")
	} else {
		fmt.Println("release")
	}
}
